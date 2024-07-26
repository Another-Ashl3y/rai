use std::io::{self, Stdout};
use crossterm::{
    cursor, style::{self, Stylize}, terminal, ExecutableCommand, QueueableCommand
};

pub struct Vec2 {
    pub x: i64,
    pub y: i64
}
impl Vec2 {
    pub fn new(x:i64, y:i64) -> Self {
        Self { x, y }
    }
}

pub struct Box {
    pub pos: Vec2,
    pub size: Vec2,
    pub strings: Vec<String>
}
impl Box {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            pos, 
            size,
            strings: Vec::new()
        }
    }
    pub fn add_string(&mut self, string:String) {
        self.strings.push(string);
    }
    pub fn draw(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout.queue(cursor::Hide)?;
        let boundary_1 = self.size.x-1;
        let boundary_2 = self.size.x-1;
        for y in 0..self.size.y {
            if y < self.strings.len() as i64 && y < self.size.y - 2 && self.strings.len() as i64 - y - 1 >= 0 {
                let text = format!(" {:<width$}", &self.strings[self.strings.len() - y as usize - 1][..(boundary_1 as usize)
                    .min(self.strings[self.strings.len() - y as usize - 1].len())], width=boundary_2 as usize);
                stdout
                    .queue(cursor::MoveTo((1+self.pos.x) as u16, (y+1+self.pos.y) as u16))?
                    .queue(style::PrintStyledContent(text.stylize()))?;
            }
            for x in 0..self.size.x {
                let top = y == 0;
                let bottom = y == self.size.y - 1;
                let left = x == 0;
                let right = x == self.size.x - 1;
                if top || bottom || left || right {
                    let mut character = "━";
                    if top && left {
                        character = "┏"
                    }
                    else if bottom && left {
                        character = "┗"
                    }
                    else if bottom && right {
                        character = "┛"
                    }
                    else if top && right {
                        character = "┓"
                    }
                    else if left || right {
                        character = "┃"
                    }
                    stdout
                        .queue(cursor::MoveTo((self.pos.x + x) as u16, (self.pos.y + y) as u16))?
                        .queue(style::PrintStyledContent(character.grey()))?;
                }

            }
        }
        Ok(())
    }
    pub fn resize(&mut self, new_size: Vec2) {
        self.size = new_size;
    }    
    pub fn reposition(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
}

pub fn clear(mut stdout: &Stdout) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

