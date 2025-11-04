use core::cmp::min;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    location: Position,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            location: Position { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! \r\n")?;
        } else {
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.location)?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                // Left, Down, Right, Up, Page Up, Page Down, Home, End
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    Self::move_cursor(self, *code);
                }
                c => {
                    println!("{c}");
                }
            }
        }
    }

    fn move_cursor(&mut self, code: KeyCode) {
        let size = Terminal::size().unwrap();
        match code {
            KeyCode::Left => self.location.x = self.location.x.saturating_sub(1),
            KeyCode::Right => {
                self.location.x = min(self.location.x.saturating_add(1), size.width - 1);
            }
            KeyCode::Down => {
                self.location.y = min(self.location.y.saturating_add(1), size.height - 1);
            }
            KeyCode::Up => {
                self.location.y = self.location.y.saturating_sub(1);
            }
            KeyCode::PageUp => {
                self.location.y = 0;
            }
            KeyCode::PageDown => {
                self.location.y = size.height - 1;
            }
            KeyCode::Home => {
                self.location.x = 0;
            }
            KeyCode::End => {
                self.location.x = size.width - 1;
            }
            _ => (),
        }
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_row()?;

            if current_row == height / 3 {
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

    fn draw_welcome_message() -> Result<(), Error> {
        let width = Terminal::size()?.width as usize;
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let padding = (width - (welcome_message.len())) / 2;
        let spaces = " ".repeat(padding - 1);

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(welcome_message)?;

        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        Self::draw_rows()?;

        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;

            self.evaluate_event(&event);
        }

        Ok(())
    }
}
