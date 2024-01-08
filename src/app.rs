use crate::todo::{Todo, Priority, Category};
use crate::views::add::{AddViewState};

pub enum View {
    List,
    Add,
    Edit,
}

pub struct App {
    pub view: View,
    pub todos: Vec<Todo>,
    pub should_quit: bool,
    pub insert_mode: bool,
    pub focused_todo: i32,
    pub add_view_state: AddViewState,
}

impl App {
    pub fn new() -> App {
        App {
            view: View::List,
            todos: Vec::new(),
            should_quit: false,
            insert_mode: false,
            focused_todo: 0,
            add_view_state: AddViewState::new(),
        }
    }

    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn add_todo(&mut self, name: String, category: Option<Category>, priority: Priority) {
       self.todos.push(Todo::new(name, category, priority, self.todos.len().try_into().unwrap())); 
    }

    pub fn up(&mut self) {
        if self.focused_todo != 0 {
            self.focused_todo -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.focused_todo != (self.todos.len() - 1).try_into().unwrap() {
            self.focused_todo += 1;
        }
    }

    pub fn toggle_insert_mode(&mut self) {
        self.insert_mode = !self.insert_mode;
    }
}
