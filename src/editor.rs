use crossterm::{
    cursor,
    style::Print,
    terminal::{self, size},
    ExecutableCommand, QueueableCommand, Result, 
    event::{KeyEvent, KeyModifiers, KeyCode, Event, read},
};
use errno::*;
use std::io::{stdout, Stdout, Write};
 

pub struct Editor {
    width: u16,
    height: u16,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (columns, rows) = size().unwrap();
        Ok(Self {
            width: columns,
            height: rows,
        })
    }

    pub fn process_keypress(&self) -> bool {
        let c = self.read_key();
        println!("{c:?}");
        match c {
            Ok(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::ALT,
                ..
            }) => return true,
            _ => return false,
        }
    }

    pub fn read_key(&self) -> Result<KeyEvent> {
        loop {
            if let Ok(event) = read() {
                if let Event::Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                self.die("read");
                break;
            }
        }
        unreachable!();
    }
    
    pub fn draw_rows(&self, stdout: &mut Stdout) -> Result<()> {
        for col in 0..24 {
            stdout
                .queue(cursor::MoveTo(0, col))?
                .queue(Print("~".to_string()))?;
        }
        Ok(())
    }

    pub fn clear_screen(&self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()
    }

    pub fn refresh_screen(&self) -> Result<()> {
        let mut stdout = stdout();

        let _ = self.clear_screen(&mut stdout);
        let _ = self.draw_rows(&mut stdout);

        stdout.queue(cursor::MoveTo(0, 0))?.flush()
    }

    /// if terminal::read() goes wrong, we can bail out and reset terminal settings here.
    pub fn die<S: Into<String>>(&self,message: S) {
        let mut stdout = stdout();
        let _ = self.clear_screen(&mut stdout);

        let _ = terminal::disable_raw_mode();

        let e = errno();
        set_errno(e);
        eprintln!("{}:  {}", message.into(), e);

        if e.0 == 0 {
            std::process::exit(0);
        }
        std::process::exit(1);
    }

}
