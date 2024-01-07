use crate::app::App;
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;

use crate::layout::layout;

pub fn list_view(frame: &mut Frame, app: &mut App) -> Result<()> {
    let horizontal_layout = layout(frame);
    let todo_height = 5;
    let todo_area_height = app.todos.len() * todo_height;
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Length(5),
            Constraint::Length(todo_area_height.try_into().unwrap()),
            Constraint::Min(0)
        ]).split(horizontal_layout[1]);

    frame.render_widget(Paragraph::new("Todo App").alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD)), layout[0]);

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
    Ok(())
}
