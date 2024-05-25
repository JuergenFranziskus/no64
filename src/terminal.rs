use std::io::{self, Write};

use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;

pub const WIDTH: usize = 200;
pub const HEIGHT: usize = 60;
pub const CHARS: usize = WIDTH * HEIGHT;

pub struct Terminal<O: Write> {
    out: O,
    cursor: [usize; 2],
    front: Box<[char; CHARS]>,
    back: Box<[char; CHARS]>,
}
impl<O: Write> Terminal<O> {
    pub fn init(mut out: O) -> io::Result<Self> {
        out.execute(Clear(ClearType::All))?;
        out.execute(Hide)?;

        Ok(Self {
            out,
            cursor: [0; 2],
            front: Box::new([' '; CHARS]),
            back: Box::new([' '; CHARS]),
        })
    }

    pub fn print(&mut self) -> io::Result<()> {
        std::mem::swap(&mut self.front, &mut self.back);

        for y in 0..HEIGHT {
            self.out.queue(MoveTo(0, y as u16))?;
            let mut cursor = (0, y);

            for x in 0..WIDTH {
                let i = y * WIDTH + x;
                let new = self.front[i];
                let old = self.back[i];
                if new != old {
                    if cursor != (x, y) {
                        self.out.queue(MoveTo(x as u16, y as u16))?;
                        cursor = (x, y);
                    }
                    self.out.queue(Print(new))?;
                    cursor.0 += 1;
                }
            }
        }
        self.out.flush()?;

        for c in self.back.iter_mut() {
            *c = ' ';
        }

        Ok(())
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.cursor[0] = x;
        self.cursor[1] = y;
    }
    pub fn put(&mut self, x: usize, y: usize, c: char) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        };
        let i = y * WIDTH + x;
        self.back[i] = c;
    }
    pub fn put_at_cursor(&mut self, c: char) {
        self.put(self.cursor[0], self.cursor[1], c);
        self.cursor[0] += 1;
    }
    pub fn write(&mut self, text: &str) {
        for c in text.chars() {
            self.put_at_cursor(c);
        }
    }
}
