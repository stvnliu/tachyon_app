mod gruvbox;

use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    style::Color,
};
use ratatui::{
    self,
    prelude::CrosstermBackend,
    style::Style,
    widgets::{Block, Widget},
    Frame,
};
use tachyonfx::CenteredShrink;

/*
* This part needs a bit of explanation
* What is a Box<dyn Error>?
*
* */
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Raterminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

fn tui(f: &mut Frame) {
    let frame_w = f.size().width;
    let frame_h = f.size().height;
    Block::default()
        .style(Style::default().bg(ratatui::style::Color::Blue))
        .render(f.area(), f.buffer_mut());
    let content_area = f.area().inner_centered(frame_w - 4, frame_h - 4);
    Block::default()
        .style(Style::default().bg(ratatui::style::Color::Red))
        .render(content_area, f.buffer_mut());
}

fn run(terminal: &mut Raterminal) -> io::Result<()> {
    loop {
        // NOTE frame is assumed to be borrowed here with pipe operators??
        terminal.draw(tui)?;
        if event::poll(std::time::Duration::from_millis(32))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(()),
                        _ => todo!(),
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    if let Err(val) = result {
        println!("{val:?}");
    }
    Ok(())
    // demo::run_it();
}
