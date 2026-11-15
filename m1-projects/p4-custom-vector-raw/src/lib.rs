use std::{
    alloc::{Layout, alloc, handle_alloc_error, realloc},
    marker::PhantomData,
    ptr::NonNull,
};

/// Low-level memory buffer for `CustomVec<T>`.
pub struct RawVec<T> {
    /// Heap buffer pointer. NonNull ensures non-zero and correct variance.
    ptr: NonNull<T>,
    /// Allocated capacity (in number of elements, not bytes).
    cap: usize,
    /// Tells the compiler we logically own `T` - for drop check and variance.
    _marker: PhantomData<T>,
}

impl<T> RawVec<T> {
    /// Creates a new, empty `RawVec`.
    ///
    /// Does not allocate any heap memory until elements are actually pushed.
    pub fn new() -> Self {
        // Must have logic for ZST (Zero-Sized Types) and empty vectors
        let cap = if std::mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };

        Self {
            ptr: std::ptr::NonNull::dangling(),
            cap,
            _marker: std::marker::PhantomData,
        }
    }

    /// Allocates or grows the buffer to accomodate more elements.
    pub fn grow(&mut self) {
        // ZST do not need heap allocation.
        if size_of::<T>() == 0 {
            return;
        }

        // Calculate new capacity
        let (new_cap, new_layout) = if self.cap == 0 {
            (4, Layout::array::<T>(4).unwrap())
        } else {
            // Check for capacity overflow
            let new_cap = self.cap.checked_mul(2).expect("capacity overflow");
            let layout = Layout::array::<T>(new_cap).expect("capacity overflow");
            (new_cap, layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation size too large"
        );

        // Allocate or reallocate memory on the heap
        let new_ptr = if self.cap == 0 {
            // First time allocation
            unsafe { alloc(new_layout) }
        } else {
            // Reallocate existing buffer
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // Handle Out-Of-Memory errors
        let new_ptr =
            NonNull::new(new_ptr as *mut T).unwrap_or_else(|| handle_alloc_error(new_layout));

        // Update state
        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}


impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = std::mem::size_of::<T>();

        if self.cap > 0 && elem_size != 0 {
            let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

/// A contiguous, growable array type implemented from scratch.
pub struct CustomVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> CustomVec<T> {
    /// Creates a new, empty `CustomVec`.
    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends an element to the back of the collection.
    pub fn push(&mut self, elem: T) {
        if self.len == self.buf.cap {
            self.buf.grow();
        }

        unsafe {
            let end_ptr = self.buf.ptr.as_ptr().add(self.len);
            std::ptr::write(end_ptr, elem);
        }

        self.len += 1;
    }

    /// Removes the last element from the vector and return it, or `None` if it's empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe  {
                let elem_ptr = self.buf.ptr.as_ptr().add(self.len);
                Some(std::ptr::read(elem_ptr))
            }
        }
    }
    
}


impl<T> Drop for CustomVec<T>{
    // Call pop() till vector is not empty
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_vec_allocation_and_growth() {
        let mut raw: RawVec<i32> = RawVec::new();
        assert_eq!(raw.cap, 0);

        raw.grow();
        assert_eq!(raw.cap, 4);

        raw.grow();
        assert_eq!(raw.cap, 8);
    }

    #[test]
    fn test_zero_sized_types() {
        let mut raw: RawVec<()> = RawVec::new();
        assert_eq!(raw.cap, usize::MAX);

        raw.grow();
        assert_eq!(raw.cap, usize::MAX);
    }

    #[test]
    fn test_custom_vec_push_pop() {
        let mut vec = CustomVec::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);

        vec.push(10);
        vec.push(20); 
        vec.push(30);
        
        assert_eq!(vec.len(), 3);
        assert!(!vec.is_empty());
        
        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.len(), 1);
        
        vec.push(40);
        assert_eq!(vec.pop(), Some(40));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
        assert!(vec.is_empty());
    }
}
