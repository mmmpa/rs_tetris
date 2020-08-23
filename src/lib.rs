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
pub const FIELD_H: usize = DISPLAY_FIELD_H + 3;
