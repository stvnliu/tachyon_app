mod gruvbox;

mod ui;

use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::CrosstermBackend;
/*
* This part needs a bit of explanation
* What is a Box<dyn Error>?
*
* */
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Raterminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

fn run(terminal: &mut Raterminal) -> io::Result<()> {
    loop {
        // NOTE frame is assumed to be borrowed here with pipe operators??
        terminal.draw(ui::build_ui::tui)?;
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
