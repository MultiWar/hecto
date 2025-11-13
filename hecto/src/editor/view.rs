use std::io::Error;

use super::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use super::buffer::Buffer;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        Terminal::clear_row()?;

        for current_row in 0..height {
            Terminal::clear_row()?;

            if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
            } else if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }

            Terminal::execute()?;
        }

        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let width = Terminal::size()?.width as usize;
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let padding = (width - (welcome_message.len())) / 2;
        let spaces = " ".repeat(padding - 1);

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(&welcome_message)?;

        Ok(())
    }
}
