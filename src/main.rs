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

pub mod app;
pub mod todo;
pub mod views;
pub mod layout;

use app::{App, View};
use todo::{Todo, Priority};
use views::list::list_view;
use views::add::add_view;

fn main() -> Result<()> {
    startup()?;
    let _result = run();
    shutdown()?;
    Ok(())
}

fn startup() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
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

            match app.view {
                View::List => {
                    let _list_view = list_view(frame, &mut app);
                },
                View::Add => {
                    let _add_view = add_view(frame, &mut app);
                },
                View::Edit => {},
            }
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        Char('q') => app.should_quit = true,
                        Char('j') => app.down(),
                        Char('k') => app.up(),
                        Char('c') => {
                           let active_todo: &mut Todo = &mut app.todos[<i32 as TryInto<usize>>::try_into(app.focused_todo).unwrap()];
                            if active_todo.completed {
                                active_todo.mark_incomplete();
                            } else {
                                active_todo.mark_complete();
                            }
                        },
                        Char('a') => app.set_view(View::Add),
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
