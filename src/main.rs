pub mod game;
pub mod world;
use std::io::{self};
use termion::{raw::IntoRawMode, screen::AlternateScreen};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let mut stdout = AlternateScreen::from(stdout);

    // start game loop
    game::start(&mut stdout)?;

    Ok(())
}
