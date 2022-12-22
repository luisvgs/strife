use crate::document::*;
use crate::position::Position;
use crate::row::*;
use crate::terminal::*;
use std::env;
use termion::color;
use termion::event::Key;

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

pub struct Editor {
    pub quit: bool,
    pub terminal: Terminal,
    pub cursor_position: Position,
    pub document: Document,
    pub offset: Position,
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document: Document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            quit: false,
            terminal: Terminal::default().unwrap(),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
        }
    }
}

impl Editor {
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Strife editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;

        let len = welcome_message.len();

        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);

        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::cursor_position(&Position::default());
        match self.quit {
            true => {
                Terminal::clear_screen();
                println!("Goodbye. :).");
            }
            _ => {
                self.draw_rows();
                self.draw_status_bar();
                self.draw_message_bar();
                Terminal::cursor_position(&Position {
                    x: self.cursor_position.x.saturating_sub(self.offset.x),
                    y: self.cursor_position.y.saturating_sub(self.offset.y),
                });
            }
        }

        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!("{} - {} lines", file_name, self.document.len());
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_bg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
    }

    fn kill(&self, e: std::io::Error) {
        Terminal::clear_screen();
        panic!("{}", e)
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            Key::Down | Key::Up | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    pub fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
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
