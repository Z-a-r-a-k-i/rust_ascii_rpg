pub mod game;
pub mod world;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self};

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    // setup the terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    // setup to capture mouse events
    execute!(io::stdout(), EnableMouseCapture)?;

    // start game loop
    game::start(&mut stdout)?;

    // clean up the terminal
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    // stop capturing mouse events, including the mouse wheel
    execute!(io::stdout(), DisableMouseCapture)?;
    Ok(())
}
