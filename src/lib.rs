#![allow(warnings)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

mod mino;

use core::mem::replace;
pub use mino::*;

pub const FIELD_W: usize = 10;
pub const FIELD_H: usize = 20;

pub struct Field {
    row_pointers: [usize; FIELD_H],
    row_counts: [usize; FIELD_H],
    actual_rows: [[bool; FIELD_W]; FIELD_H],
}

impl Field {
    pub fn new() -> Self {
        let mut rows = [0; FIELD_H];
        for i in 0..FIELD_H {
            rows[i] = i;
        }

        Self {
            row_pointers: rows,
            row_counts: [0; FIELD_H],
            actual_rows: [[false; FIELD_W]; FIELD_H],
        }
    }

    pub fn pointer_y(&self, y: usize) -> usize {
        self.row_pointers[y]
    }

    pub fn set(&mut self, x: usize, pointer_y: usize) {
        self.actual_rows[pointer_y][x] = true;
        self.row_counts[pointer_y] += 1;
    }

    pub fn delete(&mut self, pointer_y: usize) -> bool {
        if self.row_counts[pointer_y] != FIELD_W {
            return false;
        }

        self.row_counts[pointer_y] = 0;
        self.actual_rows[pointer_y] = [false; FIELD_W];

        true
    }

    pub fn shift(&mut self, y: usize) {
        let mut now = y;
        loop {
            let above_pointer = self.row_pointers[now - 1];
            if self.row_counts[above_pointer] != 0 {
                return;
            }
            let now_pointer = self.row_pointers[now];
            self.row_pointers[now] = above_pointer;
            self.row_pointers[now - 1] = now_pointer;
            now -= 1;
        }
    }
}
