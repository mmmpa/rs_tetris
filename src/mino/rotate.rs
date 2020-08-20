use crate::*;

pub struct Right;
pub struct Left;

pub struct RotatedMino<T: Into<Mino>, LR> {
    mino: T,
    direction: LR,
    offset: Offset,
}

impl<T: Into<Mino>, LR> From<RotatedMino<T, LR>> for (Mino, Offset) {
    fn from(r: RotatedMino<T, LR>) -> Self {
        let RotatedMino { mino, offset, .. } = r;
        (mino.into(), offset)
    }
}

#[macro_export]
macro_rules! mino {
    ( $now:tt, $right:tt, $right_x:tt, $right_y:tt, $left:tt, $left_x:tt, $left_y:tt, $cells:expr ) => {
        impl $now {
            pub fn right(self) -> RotatedMino<$right, Right> {
                RotatedMino {
                    mino: $right,
                    direction: Right,
                    offset: Offset {
                        x: Length::$right_x,
                        y: Length::$right_y,
                    },
                }
            }

            pub fn left(self) -> RotatedMino<$left, Left> {
                RotatedMino {
                    mino: $left,
                    direction: Left,
                    offset: Offset {
                        x: Length::$left_x,
                        y: Length::$left_y,
                    },
                }
            }

            pub fn cells(&self) -> &[&[u8]] {
                $cells
            }
        }
    };
}

// ⬛⬜⬛⬛
// -----
// ⬛
// ⬜
// ⬛
// ⬛
// -----
// ⬛⬛⬜⬛
// -----
// ⬛
// ⬛
// ⬜
// ⬛
mino!(A0, A1, P1, M1, A3, P1, M2, &[&[1, 1, 1, 1]]);
mino!(A1, A2, M2, P1, A0, M1, P1, &[&[1], &[1], &[1], &[1]]);
mino!(A2, A3, P2, M2, A1, P2, M1, &[&[1, 1, 1, 1]]);
mino!(A3, A0, M1, P2, A2, M2, P2, &[&[1], &[1], &[1], &[1]]);

// ⬛⬛
// ⬛⬛
mino!(B0, B0, No, No, B0, No, No, &[&[1, 1], &[1, 1]]);

// 　⬛⬛
// ⬛⬜
// -----
// ⬛
// ⬜⬛
// 　⬛
// -----
// 　⬜⬛
// ⬛⬛
// -----
// ⬛
// ⬛⬜
// 　⬛
mino!(C0, C1, P1, No, C3, No, No, &[&[0, 1, 1], &[1, 1]]);
mino!(C1, C2, M1, P1, C0, M1, No, &[&[1], &[1, 1], &[0, 1]]);
mino!(C2, C3, No, M1, C1, P1, M1, &[&[0, 1, 1], &[1, 1]]);
mino!(C3, C0, No, No, C2, No, P1, &[&[1], &[1, 1], &[0, 1]]);

// ⬛⬛
// 　⬜⬛
// -----
//  ⬛
// ⬜⬛
// 　⬛
// -----
// ⬛⬜
// 　⬛⬛
// -----
// 　⬛
// ⬛⬜
// ⬛
mino!(D0, D1, P1, No, D3, No, No, &[&[1, 1], &[0, 1, 1]]);
mino!(D1, D2, M1, P1, D0, M1, No, &[&[0, 1], &[1, 1], &[1]]);
mino!(D2, D3, No, M1, D1, P1, M1, &[&[1, 1], &[0, 1, 1]]);
mino!(D3, D0, No, No, D2, No, P1, &[&[0, 1], &[1, 1], &[1]]);

// ⬛
// ⬛⬜⬛
// -----
// ⬛⬛
// ⬜
// ⬛
// -----
// ⬛⬜⬛
// 　　⬛
// -----
// 　⬛
// 　⬜
// ⬛⬛
mino!(E0, E1, P1, No, E3, No, No, &[&[1], &[1, 1, 1]]);
mino!(E1, E2, M1, P1, E0, M1, No, &[&[1, 1], &[1], &[1]]);
mino!(E2, E3, No, M1, E1, P1, M1, &[&[1, 1, 1], &[0, 0, 1]]);
mino!(E3, E0, No, No, E2, No, P1, &[&[0, 1], &[0, 1], &[1, 1]]);

// 　　⬛
// ⬛⬜⬛
// -----
// ⬛
// ⬜
// ⬛⬛
// -----
// ⬛⬜⬛
// ⬛
// -----
// ⬛⬛
// 　⬜
// 　⬛
mino!(F0, F1, P1, No, F3, No, No, &[&[0, 0, 1], &[1, 1, 1]]);
mino!(F1, F2, M1, P1, F0, M1, No, &[&[1], &[1], &[1, 1]]);
mino!(F2, F3, No, M1, F1, P1, M1, &[&[1, 1, 1], &[1]]);
mino!(F3, F0, No, No, F2, No, P1, &[&[1, 1], &[0, 1], &[0, 1]]);

// 　⬛
// ⬛⬜⬛
// -----
// ⬛
// ⬜⬛
// ⬛
// -----
// ⬛⬜⬛
// 　⬛
// -----
// 　⬛
// ⬛⬜
// 　⬛
mino!(G0, G1, P1, No, G3, No, No, &[&[0, 1], &[1, 1, 1]]);
mino!(G1, G2, M1, P1, G0, M1, No, &[&[1], &[1, 1], &[1]]);
mino!(G2, G3, No, M1, G1, P1, M1, &[&[1, 1, 1], &[0, 1]]);
mino!(G3, G0, No, No, G2, No, P1, &[&[0, 1], &[1, 1], &[0, 1]]);

#[cfg(test)]
mod tests {
    use crate::{Mino, State, A0, B0, C0, D0, E0, F0, G0};
    use std::prelude::v1::*;

    fn print_test(state: &State, w: usize, h: usize) -> String {
        let mut canvas = vec![vec![0; w]; h];

        state.mino.cells().iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, b)| canvas[state.y as usize + y][state.x as usize + x] = *b)
        });

        canvas.iter().fold("".to_string(), |mut a, row| {
            row.iter().fold(a, |a, b| a + if *b == 0 { "⬜" } else { "⬛" })
        })
    }

    #[test]
    fn offset_mino_a() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬛⬛⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜⬜⬜\
            ",
        ];

        let mut state = State::new(Mino::A0(A0), 2, 3);
        for (i, now) in table.iter().enumerate() {
            let s = print_test(&state, 7, 7);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.right();
        }

        let mut state = State::new(Mino::A0(A0), 2, 3);
        for (i, now) in table.iter().rev().enumerate() {
            let s = print_test(&state, 7, 7);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.left();
        }
    }

    #[test]
    fn offset_mino_c() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬜⬛⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬜⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
        ];

        let mut state = State::new(Mino::C0(C0), 1, 1);
        for (i, now) in table.iter().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.right();
        }

        let mut state = State::new(Mino::C0(C0), 1, 1);
        for (i, now) in table.iter().rev().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.left();
        }
    }

    #[test]
    fn offset_mino_d() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬛⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
        ];

        let mut state = State::new(Mino::D0(D0), 1, 1);
        for (i, now) in table.iter().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.right();
        }

        let mut state = State::new(Mino::D0(D0), 1, 1);
        for (i, now) in table.iter().rev().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.left();
        }
    }

    #[test]
    fn offset_mino_e() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬜⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬛⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬜⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
        ];

        let mut state = State::new(Mino::E0(E0), 1, 1);
        for (i, now) in table.iter().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.right();
        }

        let mut state = State::new(Mino::E0(E0), 1, 1);
        for (i, now) in table.iter().rev().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.left();
        }
    }

    #[test]
    fn offset_mino_f() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬛⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬛⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬛⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
        ];

        let mut state = State::new(Mino::F0(F0), 1, 1);
        for (i, now) in table.iter().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.right();
        }

        let mut state = State::new(Mino::F0(F0), 1, 1);
        for (i, now) in table.iter().rev().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.left();
        }
    }

    #[test]
    fn offset_mino_g() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬛⬛⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬛⬛⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
            "\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬛⬜⬜\
                ⬜⬛⬛⬛⬜\
                ⬜⬜⬜⬜⬜\
                ⬜⬜⬜⬜⬜\
            ",
        ];

        let mut state = State::new(Mino::G0(G0), 1, 1);
        for (i, now) in table.iter().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.right();
        }

        let mut state = State::new(Mino::G0(G0), 1, 1);
        for (i, now) in table.iter().rev().enumerate() {
            let s = print_test(&state, 5, 5);
            assert_eq!(now, &&s, "\n{} {}", i, s);
            state = state.left();
        }
    }
}
