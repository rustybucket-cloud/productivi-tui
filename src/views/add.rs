use std::collections::HashMap;

use crate::app::App;
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;
use crossterm::{
    event::{self, KeyCode, KeyCode::Char, KeyEvent, Event::Key},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use crate::layout::layout;
use crate::todo::{Priority};

pub struct AddViewState {
    name: String,
    category: String,
    priority: Priority,
    focus: i32,
    insert_mode: bool,
}

impl AddViewState {
    pub fn new() -> AddViewState {
        AddViewState {
            name: String::new(),
            category: String::new(),
            priority: Priority::P3,
            focus: 0,
            insert_mode: false,
        }
    }

    pub fn focus_down(&mut self) {
        if self.focus < 2 {
           self.focus += 1; 
        }
    }

    pub fn focus_up(&mut self) {
        if self.focus > 0 {
           self.focus -= 1; 
        }
    }

    pub fn toggle_insert_mode(&mut self) {
       self.insert_mode = !self.insert_mode;
    }

    pub fn update_input(&mut self, key: KeyEvent) {
        match self.focus {
            0 => {
                if let KeyCode::Char(c) = key.code {
                    self.name = format!("{}{}", self.name, c);
                }
            },
            1 => {
                if let KeyCode::Char(c) = key.code {
                    self.category = format!("{}{}", self.category, c);
                }
            },
            _ => {},
        }
    }
}

pub fn add_view(frame: &mut Frame, app: &mut App) -> Result<()> {
    let horizontal_layout = layout(frame);    

    let heading = Paragraph::new("Add Todo").alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD));
    let name = input(format!("{}", app.add_view_state.name), app.add_view_state.focus == 0, "Name");
    let category = input(format!("{}", app.add_view_state.category), app.add_view_state.focus == 1, "Category");
    let priority = input(
        match app.add_view_state.priority {
            Priority::P1 => String::from("P1"),
            Priority::P2 => String::from("P2"),
            Priority::P3 => String::from("P3")
        },
        app.add_view_state.focus == 2,
        "Priority"
    );

    let num_elements = 4;
    let input_height = 5;
    let mut constraints: Vec<Constraint> = Vec::new();
    for _ in 0..num_elements {
       constraints.push(Constraint::Length(input_height)); 
    }
    constraints.push(Constraint::Min(0));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints).split(horizontal_layout[1]);

    frame.render_widget(heading, layout[0]);
    frame.render_widget(name, layout[1]);
    frame.render_widget(category, layout[2]);
    frame.render_widget(priority, layout[3]);

    Ok(())
}

fn input(value: String, focused: bool, label: &str) -> Paragraph {
    Paragraph::new(format!("{}", value))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1,1,1,1))
                    .style(
                        Style::default()
                            .fg(if focused{ Color::Black } else { Color::Yellow })
                            .bg(if focused{ Color::Yellow } else { Color::default() })
                    )
                    .title(label)
            )
}
