use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;

pub fn layout(frame: &mut Frame) -> Rc<[Rect]> {
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

    return horizontal_layout;
}
