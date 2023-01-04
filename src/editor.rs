use std::io::Write;

use crossterm::{
    cursor,
    Result, 
    terminal,
    event::{KeyEvent, KeyModifiers, KeyCode}, QueueableCommand,
};  
use errno::*; 

use crate::{keyboard::*, screen::*};



pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
}

impl Editor {
    pub fn new() -> Result<Self> { 
        let s = Screen::new()?;
        let k = Keyboard;
        Ok(Self {
            screen: s,
            keyboard: k,
            
        })
    }

    pub fn start(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        loop {
            if self.refresh_screen().is_err() {
                self.die("couldn't refresh screen!");
            }
            if self.process_keypress() {
                break;
            }
        }
        let _ = terminal::disable_raw_mode();
        Ok(())
    }

    pub fn process_keypress(&mut self) -> bool {
        let c = self.keyboard.read();
        println!("{c:?}");
        match c {
            Ok(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::ALT,
                ..
            }) => true,
            Err(_) => {
                self.die("unable to read from keyboard");
                false
            } 
            _ => false,
        }
    }
    
    pub fn refresh_screen(&mut self) -> Result<()> { 
        self.screen.clear()?;
        self.screen.draw_rows()?;

        self.screen.stdout.queue(cursor::MoveTo(0, 0))?;
        self.screen.stdout.flush()
    }

    /// if terminal::read() goes wrong, we can bail out and reset terminal settings here.
    pub fn die<S: Into<String>>(&mut self, message: S) { 
        let _ = self.screen.clear(); 
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
