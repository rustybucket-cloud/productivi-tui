use crate::app::App;
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;

use crate::layout::layout;

pub fn add_view(frame: &mut Frame, app: &mut App) -> Result<()> {
    let horizontal_layout = layout(frame);    

    let input_height = 5;
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(input_height),
        ]).split(horizontal_layout[1]);

    let heading = Paragraph::new("Add Todo").alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(heading, layout[0]);

    Ok(())
}
