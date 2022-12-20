use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    pub quit: bool,
    pub terminal: Terminal,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            quit: false,
            terminal: Terminal::default().unwrap(),
        }
    }
}

impl Editor {
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                let welcome_message = format!("Strife editor -- version {}", VERSION);
                let width =
                    std::cmp::min(self.terminal.size().width as usize, welcome_message.len());
                println!("{}\r", &welcome_message[..width]);
            } else {
                println!("~\r");
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::cursor_position(0, 0);
        match self.quit {
            true => {
                Terminal::clear_screen();
                println!("Goodbye. :).");
            }
            _ => {
                self.draw_rows();
                Terminal::cursor_position(0, 0);
            }
        }

        Terminal::hide_cursor();
        Terminal::flush()
    }

    fn kill(&self, e: std::io::Error) {
        Terminal::clear_screen();
        panic!("{}", e)
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        match Terminal::read_key()? {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }

        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                self.kill(e)
            }

            if let Err(e) = self.process_keypress() {
                self.kill(e)
            }

            if self.quit {
                break;
            }
        }
    }
}
