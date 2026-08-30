
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
        todo!()
    }
        
    pub fn mark_done(&mut self, id: usize) -> Result<(), String> {
        todo!()
    }

    pub fn remove_item(&mut self, id: usize) -> Option<TodoItem> {
        todo!()
    }

    pub fn list_items(&self) -> &[TodoItem] {
        todo!()
    } 

}



