use core::cmp::min;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self},
    KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()
    }

    fn move_point(&mut self, code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

        match code {
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                x = min(x.saturating_add(1), width.saturating_sub(1));
            }
            KeyCode::Down => {
                y = min(y.saturating_add(1), height.saturating_sub(1));
            }
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
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
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
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

            self.evaluate_event(&event)?;
        }

        Ok(())
    }
}
