use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, Result};
use std::io::{stdout, Stdout, Write};
use errno::*;

pub fn clear_screen(stdout: &mut Stdout) -> Result<()> {
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

pub fn editor_refresh_screen() -> Result<()> {
    let mut stdout = stdout();
    let _ = clear_screen(&mut stdout);
    stdout.flush()?;
    Ok(())
}

/// if terminal::read() goes wrong, we can bail out and reset terminal settings here.
pub fn die<S: Into<String>>(message: S) {
    let mut stdout = stdout();
    let _ = clear_screen(&mut stdout);

    let _ = terminal::disable_raw_mode();

    let e = errno();
    set_errno(e);
    eprintln!("{}:  {}", message.into(), e);

    if e.0 == 0 {
        std::process::exit(0);
    }
    std::process::exit(1);
}
