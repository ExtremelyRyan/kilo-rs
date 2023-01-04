use crossterm::{
    cursor,
    style::Print,
    terminal::{self, size},
    ExecutableCommand, QueueableCommand, Result,
}; 
use std::io::{stdout, Stdout, Write};
 
 
#[derive(Debug)]
pub struct Screen {
    pub stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let (columns, rows) = size().unwrap();
        Ok(Self {
            width: columns,
            height: rows,
            stdout: stdout(),
        })
    }
    pub fn draw_rows(&mut self) -> Result<()> {
        for row in 0..self.height {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?;
        }
        Ok(())
    }
    pub fn clear(&mut self) -> Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()
    }
}
 