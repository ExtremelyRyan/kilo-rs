use crossterm::{terminal, Result};

mod editor;
use editor::Editor;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new()?;

    loop {
        if editor.refresh_screen().is_err() {
            editor.die("couldn't refresh screen!");
        }
        if editor.process_keypress() {
            break;
        }
    }
    let _ = terminal::disable_raw_mode();
    Ok(())
}
