use crossterm::{event::Event, terminal, Result};

mod keyboard;
use keyboard::*;
mod input;
use input::*;
mod output;
use output::*;

fn main() -> Result<()> {
    // println!("{:?}", terminal::is_raw_mode_enabled());
    terminal::enable_raw_mode()?;
    loop {
        if editor_refresh_screen().is_err() {
            die("couldn't refresh screen!");
        }
        if editor_process_keypress() {
            break;
        }
    }
    let _ = terminal::disable_raw_mode();
    Ok(())
}

 