pub struct Category {
    name: String
}

pub enum Priority {
    P1,
    P2,
    P3,
}

pub struct Todo {
    pub name: String,
    pub category: Option<Category>,
    pub priority: Priority,
    pub completed: bool,
    pub id: i32,
}

impl Todo {
    pub fn new(name: String, category: Option<Category>, priority: Priority, id: i32) -> Todo {
        Todo {
            name,
            category,
            priority,
            id,
            completed: false,
        } 
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    pub fn mark_incomplete(&mut self) {
        self.completed = false;
    }
}
