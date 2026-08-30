
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Pending,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    id: usize,
    title: String,
    status: Status,
}

pub struct TodoList {
    items: Vec<TodoItem>,
    next_id: usize,
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



