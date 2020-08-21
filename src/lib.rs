#![allow(warnings)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

mod field;
mod mino;

pub use field::*;
pub use mino::*;

pub const FIELD_W: usize = 10;
pub const DISPLAY_FIELD_H: usize = 20;
pub const FIELD_H: usize = DISPLAY_FIELD_H + 3;
