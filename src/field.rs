use crate::*;
use core::mem;

#[derive(Debug)]
pub struct Field {
    counts: [usize; FIELD_H],
    rows: [[bool; FIELD_W]; FIELD_H],
}

/// y: 0 is the bottom
/// Display must flip vertical.
impl Field {
    pub fn new() -> Self {
        let mut rows = [0; FIELD_H];
        for i in 0..FIELD_H {
            rows[i] = i;
        }

        Self {
            counts: [0; FIELD_H],
            rows: [[false; FIELD_W]; FIELD_H],
        }
    }

    pub fn test(&self, x: usize, y: usize) -> bool {
        self.rows[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.rows[y][x] = true;
        self.counts[y] += 1;
    }

    pub fn delete(&mut self, y: usize) -> bool {
        if self.counts[y] != FIELD_W {
            return false;
        }

        self.counts[y] = 0;
        self.rows[y] = [false; FIELD_W];

        true
    }

    // MUST float from above after delete multiline
    pub fn float(&mut self, y: usize) {
        let mut now = y;
        while now < FIELD_H {
            let up_row = now + 1;
            if self.counts[up_row] == 0 {
                return;
            }

            self.rows.swap(now, up_row);
            self.counts.swap(now, up_row);

            now += 1;
        }
    }
}

#[cfg(test)]
mod filed_tester {
    use crate::*;
    use std::prelude::v1::*;

    pub fn print_field(f: &Field, h: usize) -> String {
        let mut canvas = vec![vec!["⬜"; FIELD_W]; h];

        for y in 0..h {
            for x in 0..FIELD_W {
                if f.test(x, y) {
                    canvas[y as usize][x as usize] = "⬛";
                }
            }
        }

        canvas
            .into_iter()
            .rev()
            .flat_map(|mut s| {
                s.push("\n");
                s
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::field::filed_tester::print_field;
    use crate::*;
    use std::prelude::v1::*;

    #[test]
    fn test_float() {
        let mut f = Field::new();

        f.set(0, 0);
        f.set(3, 1);
        f.set(4, 1);
        f.set(2, 2);

        let s = print_field(&f, 4);
        assert_eq!([1, 2, 1, 0], f.counts[0..4]);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜\n\
                ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
            ",
            s,
            "{}",
            s
        );

        f.float(1);

        let s = print_field(&f, 4);
        assert_eq!([1, 1, 2, 0], f.counts[0..4]);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜\n\
                ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
            ",
            s
        );

        f.float(0);

        let s = print_field(&f, 4);
        assert_eq!([1, 2, 1, 0], f.counts[0..4]);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜\n\
            ",
            s
        );
    }

    #[test]
    fn test_delete() {
        let mut f = Field::new();

        for x in 0..FIELD_W {
            f.set(x, 0);
        }
        f.set(3, 1);

        let s = print_field(&f, 3);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜\n\
                ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛\n\
            ",
            s
        );

        assert_eq!(false, f.delete(1));
        let s = print_field(&f, 3);
        assert_eq!(1, f.counts[1]);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜\n\
                ⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛\n\
            ",
            s
        );

        assert_eq!(true, f.delete(0));
        let s = print_field(&f, 3);
        assert_eq!(0, f.counts[0]);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
            ",
            s
        );
    }
}
