use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::position::Position;

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn show_cursor() {
        print!("{}", termion::cursor::Show)
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide)
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_position(pos: &Position) {
        let Position { mut x, mut y } = pos;
        x = pos.x.saturating_add(1);
        y = pos.y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
