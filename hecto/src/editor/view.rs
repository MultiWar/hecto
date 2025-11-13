use std::io::Error;

use super::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_row()?;

            if current_row == 0 {
                Self::draw_welcome_message(Some("Hello, World!"))?;
            } else if current_row == height / 3 {
                Self::draw_welcome_message(None)?;
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

    fn draw_welcome_message(custom_message: Option<&str>) -> Result<(), Error> {
        let width = Terminal::size()?.width as usize;

        let custom_message_str = custom_message.unwrap_or("");
        let mut welcome_message: String;

        if custom_message_str != "" {
            welcome_message = String::from(custom_message_str);
        } else {
            welcome_message = format!("{NAME} editor -- version {VERSION}");
        }

        let padding = (width - (welcome_message.len())) / 2;
        let spaces = " ".repeat(padding - 1);

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(&welcome_message)?;

        Ok(())
    }
}
