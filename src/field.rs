use crate::*;
use core::mem;

#[derive(Default, Debug)]
pub struct Field {
    counts: [usize; FIELD_H],
    rows: [[bool; FIELD_W]; FIELD_H],
}

/// y: 0 is the bottom
/// Display must flip vertical.
impl Field {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test(&self, x: i8, y: i8) -> bool {
        let x = x as usize;
        let y = y as usize;

        if x < 0 || FIELD_W <= x || y < 0 || FIELD_H <= y {
            return true;
        }

        self.rows[y][x]
    }

    pub fn set(&mut self, x: i8, y: i8) {
        self.rows[y as usize][x as usize] = true;
        self.counts[y as usize] += 1;
    }

    pub fn is_filled(&self, y: i8) -> bool {
        self.counts[y as usize] == FIELD_W
    }

    pub fn delete(&mut self, y: i8) -> bool {
        if self.counts[y as usize] != FIELD_W {
            return false;
        }

        self.counts[y as usize] = 0;
        self.rows[y as usize] = [false; FIELD_W];

        true
    }

    /// MUST float from above after delete multiline
    pub fn float(&mut self, y: i8) {
        let mut now = y;
        while now > 0 {
            let up_row = now - 1;
            if self.counts[up_row as usize] == 0 {
                return;
            }

            self.rows.swap(now as usize, up_row as usize);
            self.counts.swap(now as usize, up_row as usize);

            now -= 1;
        }
    }

    pub fn rows(&self) -> &[[bool; FIELD_W]; FIELD_H] {
        &self.rows
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
                if f.test(x as i8, y as i8) {
                    canvas[y as usize][x as usize] = "⬛";
                }
            }
        }

        canvas
            .into_iter()
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

        f.set(2, 1);
        f.set(4, 2);
        f.set(3, 2);
        f.set(0, 3);

        let s = print_field(&f, 4);
        assert_eq!([0, 1, 2, 1], f.counts[0..4]);
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

        f.float(2);

        let s = print_field(&f, 4);
        assert_eq!([0, 2, 1, 1], f.counts[0..4]);
        assert_eq!(
            "\
                ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬛⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜\n\
                ⬛⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
            ",
            s
        );

        f.float(3);

        let s = print_field(&f, 4);
        assert_eq!([0, 1, 2, 1], f.counts[0..4]);
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

        for x in 0..(FIELD_W as i8) {
            f.set(x, 2);
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

        assert_eq!(true, f.delete(2));
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
