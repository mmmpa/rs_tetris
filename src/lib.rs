#![allow(warnings)]
#![no_std]
#![feature(concat_idents)]

#[cfg(test)]
macro_rules! my_print {
    ($($arg:tt)*) => (println!($($arg)*));
}

#[cfg(not(test))]
macro_rules! my_print {
    ($($arg:tt)*) => {};
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_export]
mod macros;

mod field;
mod game;
mod mino;

pub use field::*;
pub use game::*;
pub use mino::*;

pub const FIELD_W: usize = 10;
pub const DISPLAY_FIELD_H: usize = 20;

// 0 is rotation space
// 1 is start position
// 2 is top of display.
pub const FIELD_H: usize = DISPLAY_FIELD_H + 2;

pub const MINO_FIRST_POSITION: (i8, i8) = (4, 1);
pub const FIELD_TOP: i8 = 2;

pub const LOCKING_TIME: u8 = 2;
