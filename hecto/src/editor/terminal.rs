use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use std::io::{stdout, Write, Error};

pub struct Terminal {}
pub struct Size {
    pub width: u16,
    pub height: u16
}

pub struct Position {
    pub x: u16,
    pub y: u16
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x: 0, y: 0})?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    pub fn clear_row() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;

        Ok(Size{ width, height })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }
}