#![allow(warnings)]

mod mino;

pub use mino::*;

pub struct State {
    mino: Mino,
    x: i8,
    y: i8,
}

impl State {
    fn new(mino: Mino, x: i8, y: i8) -> Self {
        Self { mino, x, y }
    }

    pub fn left(self) -> Self {
        let (mino, offset) = self.mino.left();
        Self {
            mino,
            x: self.x + offset.x as i8,
            y: self.y + offset.y as i8,
        }
    }

    pub fn right(self) -> Self {
        let (mino, offset) = self.mino.right();
        Self {
            mino,
            x: self.x + offset.x as i8,
            y: self.y + offset.y as i8,
        }
    }
}

pub struct Offset {
    pub x: Length,
    pub y: Length,
}

#[repr(i8)]
pub enum Length {
    M2 = -2,
    M1 = -1,
    No = 0,
    P1 = 1,
    P2 = 2,
}
