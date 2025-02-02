use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::cursor::{Hide, Show};
use crossterm::style::Print;
use std::io::stdout;
use std::io::Write;

mod terminal;
use terminal::{Size, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        queue!(stdout, Hide)?;

        if self.should_quit {
            Terminal::clear_screen()?;
            queue!(stdout, Show)?;
            queue!(stdout, Print("Goodbye! \r\n"))?;
        } else {
            Self::draw_rows()?;
            queue!(stdout, Show)?;
            Terminal::move_cursor_to(0, 0)?;
        }

        stdout.flush()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code, modifiers, .. }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => ()
            }
        }
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, ..} = Terminal::size()?;

        let mut stdout = stdout();

        for current_row in 0 .. height {
            Terminal::clear_row()?;
            queue!(stdout, Print("~"))?;

            if current_row + 1 < height {
                queue!(stdout, Print("\r\n"))?;
            }

            stdout.flush()?;
        }

        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
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