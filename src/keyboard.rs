use crossterm::event::{KeyEvent, Event, read};
use kilo_rs::*;
 
#[derive(Debug)]
pub struct Keyboard;

impl Keyboard { 
    pub fn read(&mut self) -> Result<KeyEvent,EditorResult> {
        loop {
            if let Ok(event) = read() {
                if let Event::Key(key_event) = event {
                    return Ok(key_event);
                }
            } else {
                return Err(EditorResult::KeyReadFail) 
            }
        }
    }
}

 

 