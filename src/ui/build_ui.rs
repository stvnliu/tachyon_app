use ratatui::{
    self,
    layout::Rect,
    style::Style,
    widgets::{Block, Widget},
    Frame,
};
use tachyonfx::CenteredShrink;

pub fn tui(f: &mut Frame) {
    let frame_w = f.area().width;
    let frame_h = f.area().height;
    Block::default()
        .style(Style::default().bg(ratatui::style::Color::Blue))
        .render(f.area(), f.buffer_mut());
    let content_area = f.area().inner_centered(frame_w - 2, frame_h - 2);
    Block::default()
        .style(Style::default().bg(ratatui::style::Color::Red))
        .render(content_area, f.buffer_mut());
    let inner_content_area = Rect {
        x: 3,
        y: 3,
        width: content_area.x + content_area.width - 4,
        height: 4,
    };
    Block::default()
        .style(Style::default().bg(ratatui::style::Color::Black))
        .render(inner_content_area, f.buffer_mut());
}
