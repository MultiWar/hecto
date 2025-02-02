use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use crossterm::cursor::MoveTo;
use crossterm::queue;
use std::io::stdout;
use std::io::Write;

pub struct Terminal {}
pub struct Size {
    pub row: u16,
    pub height: u16
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;
        stdout.flush()
    }

    pub fn clear_row() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::CurrentLine))?;
        stdout.flush()
    }

    pub fn move_cursor_to(column: u16, row: u16) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(column, row))?;
        stdout.flush()
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let size = size()?;

        Ok(Size{row: size.0, height: size.1})
    }
}