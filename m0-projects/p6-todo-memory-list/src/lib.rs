
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Pending,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub status: Status,
}

pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_item(&mut self, title: &str) -> &TodoItem {
        let item = TodoItem {
            id: self.next_id,
            title: title.to_string(),
            status: Status::Pending,
        };

        self.next_id += 1;
        self.items.push(item);

        self.items.last().unwrap()
    }
        
    pub fn mark_done(&mut self, id: usize) -> Result<(), String> {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.status = Status::Done;
            Ok(())
        } else {
            Err (format!("Task with ID {} not found", id))
        }
    }

    pub fn remove_item(&mut self, id: usize) -> Option<TodoItem> {
        let index = self.items.iter().position(|item| item.id == id)?;
        Some(self.items.remove(index))
    }

    pub fn list_items(&self) -> &[TodoItem] {
        &self.items
    } 

}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_add_item() {
        let mut list = TodoList::new();
        let item = list.add_item("Buy milk");

        assert_eq!(item.id, 1);
        assert_eq!(item.title, "Buy milk");
        assert_eq!(item.status, Status::Pending);
        assert_eq!(list.list_items().len(), 1);
    }

    #[test]
    fn test_mark_done() {
        let mut list = TodoList::new();
        list.add_item("Read Rust Book");

        assert!(list.mark_done(1).is_ok());
        assert_eq!(list.list_items()[0].status, Status::Done);

        assert!(list.mark_done(999).is_err());
    }

    #[test]
    fn test_remove_item() {
        let mut list = TodoList::new();
        list.add_item("First Todo Item");
        list.add_item("Second Todo Item");

        let removed = list.remove_item(1);

        assert!(removed.is_some());

        assert_eq!(removed.unwrap().title, "First Todo Item");
        assert_eq!(list.list_items().len(), 1);
    }

    #[test]
    fn test_auto_increment() {
        let mut list = TodoList::new();
        
        let item1 = list.add_item("Task 1");
        assert_eq!(item1.id, 1);

        let item2 = list.add_item("Task 2");
        assert_eq!(item2.id, 2);
    }


}

