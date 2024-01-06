use crossterm::{
    event::{self, KeyCode::Char, Event::Key},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use std::io::{stdout, Result};
use ratatui::{prelude::*, widgets::*};

fn main() -> Result<()> {
    startup()?;
    let result = run();
    shutdown()?;
    Ok(())
}

fn startup() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
}

enum View {
    List,
    Add,
    Edit,
}

struct Category {
    name: String
}

enum Priority {
    P1,
    P2,
    P3,
}

struct Todo {
    name: String,
    category: Option<Category>,
    priority: Priority,
    completed: bool,
    id: i32,
}

impl Todo {
    fn new(name: String, category: Option<Category>, priority: Priority, id: i32) -> Todo {
        Todo {
            name,
            category,
            priority,
            id,
            completed: false,
        } 
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }

    fn mark_incomplete(&mut self) {
        self.completed = false;
    }
}

struct App {
    view: View,
    todos: Vec<Todo>,
    should_quit: bool,
    focused_todo: i32,
}

impl App {
    fn new() -> App {
        App {
            view: View::List,
            todos: Vec::new(),
            should_quit: false,
            focused_todo: 0,
        }
    }

    fn set_view(&mut self, view: View) {
        self.view = view;
    }

    fn add_todo(&mut self, name: String, category: Option<Category>, priority: Priority) {
       self.todos.push(Todo::new(name, category, priority, self.todos.len().try_into().unwrap())); 
    }

    fn up(&mut self) {
        if self.focused_todo != 0 {
            self.focused_todo -= 1;
        }
    }

    fn down(&mut self) {
        if self.focused_todo != (self.todos.len() - 1).try_into().unwrap() {
            self.focused_todo += 1;
        }
    }
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut app = App::new();

    app.add_todo("Example Todo".to_string(), None, Priority::P1);
    app.add_todo("Wow this is weird".to_string(), None, Priority::P1);
    app.add_todo("How are you?".to_string(), None, Priority::P1);

    loop {
        terminal.draw(|frame| {
            let todo_height = 5;

            let content_max_width = 100;
            let frame_width = frame.size().width;
            let content_width = if frame_width < content_max_width { frame_width } else { content_max_width };
            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length((frame_width - content_width) / 2),
                    Constraint::Length(content_width),
                    Constraint::Length((frame_width - content_width) / 2)
                ]).split(frame.size());
            frame.render_widget(Block::default(), horizontal_layout[0]);
            frame.render_widget(Block::default(), horizontal_layout[2]);

            let todo_area_height = app.todos.len() * todo_height;
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(vec![
                    Constraint::Length(5),
                    Constraint::Length(todo_area_height.try_into().unwrap()),
                    Constraint::Min(0)
                ]).split(horizontal_layout[1]);

            frame.render_widget(Paragraph::new("Todo App"), layout[0]);

            let mut todo_constaints: Vec<Constraint> = Vec::new();
            for _todo in &app.todos {
               todo_constaints.push(Constraint::Max(todo_height.try_into().unwrap())); 
            }
            let todos_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(todo_constaints)
                .split(layout[1]);
            for (index, todo) in app.todos.iter().enumerate() {
                let todo_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .horizontal_margin(3)
                    .constraints([
                        Constraint::Length(10),
                        Constraint::Length(5),
                        Constraint::Min(5)
                    ])
                    .split(todos_layout[index]);
                
                frame.render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .style(Style::default().bg(if todo.completed { Color::Yellow } else { Color::default() }).fg(if todo.completed { Color::Black } else { Color::Yellow })),
                    todo_layout[0]
                );

                frame.render_widget(Block::default(), todo_layout[1]);

                let is_selected = index == app.focused_todo.try_into().unwrap();

                let p = Paragraph::new(format!("{}", todo.name))
                    .style(Style::default().fg(if is_selected { Color::Black } else { Color::Yellow }))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .padding(Padding::new(1,1,1,1))
                            .style(Style::default().bg(if is_selected { Color::Yellow } else { Color::default() }))
                    );
                frame.render_widget(p, todo_layout[2]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        Char('q') => app.should_quit = true,
                        Char('j') => app.down(),
                        Char('k') => app.up(),
                        Char('a') => {
                           let active_todo: &mut Todo = &mut app.todos[<i32 as TryInto<usize>>::try_into(app.focused_todo).unwrap()];
                            if active_todo.completed {
                                active_todo.mark_incomplete();
                            } else {
                                active_todo.mark_complete();
                            }
                        },
                        _ => {},
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn shutdown() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
