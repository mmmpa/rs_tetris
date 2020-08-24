use crate::*;

/// Return state after turning right and offsets list for retry in failure.
pub trait Right: MinoCore {
    type Next: MinoCore<
            Form = Self::Form,
            Now = Self::Right,
            Right = Self::Side,
            Side = Self::Left,
            Left = Self::Now,
        > + Right
        + Left;
    type Srs: SrsOffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Right>;

    fn right(&self) -> (Self::Next, &[(i8, i8)]) {
        let next = Self::Next::new_with_t(self.pos());
        let srs = Self::Srs::offset();

        (next, srs)
    }
}

/// Return state after turning left and offsets list for retry in failure.
pub trait Left: MinoCore {
    type Next: MinoCore<
            Form = Self::Form,
            Now = Self::Left,
            Right = Self::Now,
            Side = Self::Right,
            Left = Self::Side,
        > + Right
        + Left;
    type Srs: SrsOffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Left>;

    fn left(&self) -> (Self::Next, &[(i8, i8)]) {
        let next = Self::Next::new_with_t(self.pos());
        let srs = Self::Srs::offset();

        (next, srs)
    }
}

#[cfg(test)]
mod tests {
    use crate::mino::rotation::*;
    use crate::{BarTypeMino, State0};
    use std::prelude::v1::*;

    macro_rules! test_rendering {
        ( $table:tt, $mino:tt, $form:tt, $x:tt, $y:tt, $canvas_w:tt, $canvas_h:tt ) => {
            let mut it = $table.iter();

            let mino = MinoState::<$mino, $form, State0>::new_with($x, $y);

            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.right();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.right();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.right();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.right();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            //

            let mut it = $table.iter().rev();

            let mino = MinoState::<$mino, $form, State0>::new_with($x, $y);

            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.left();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.left();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.left();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);

            let (mino, _) = mino.left();
            let now = it.next().unwrap();
            let s = print_test(&mino, $canvas_w, $canvas_h);
            assert_eq!(now, &&s, "\n{}", s);
        };
    }

    fn print_test(state: &impl MinoCore, w: usize, h: usize) -> String {
        let mut canvas = vec![vec!["⬜"; w]; h];

        state.mut_with_absolute_cells(|x, y| {
            canvas[y as usize][x as usize] = "⬛";
        });

        canvas
            .into_iter()
            .flat_map(|mut s| {
                s.push("\n");
                s
            })
            .collect::<Vec<_>>()
            .join("")
    }

    #[test]
    fn offset_mino_a() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜⬜\n\
            ",
        ];

        test_rendering!(table, MinoI, BarTypeMino, 2, 2, 6, 6);
    }
    #[test]
    fn offset_mino_c() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬜⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬜⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
        ];

        test_rendering!(table, MinoS, NormalTypeMino, 2, 2, 5, 5);
    }

    #[test]
    fn offset_mino_d() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬛⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
        ];

        test_rendering!(table, MinoZ, NormalTypeMino, 2, 2, 5, 5);
    }

    #[test]
    fn offset_mino_e() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬜⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬜⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
        ];

        test_rendering!(table, MinoJ, NormalTypeMino, 2, 2, 5, 5);
    }

    #[test]
    fn offset_mino_f() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬛⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬛⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
        ];

        test_rendering!(table, MinoL, NormalTypeMino, 2, 2, 5, 5);
    }

    #[test]
    fn offset_mino_g() {
        let table = [
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬛⬛⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬛⬛⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
            "\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬛⬜⬜\n\
                ⬜⬛⬛⬛⬜\n\
                ⬜⬜⬜⬜⬜\n\
                ⬜⬜⬜⬜⬜\n\
            ",
        ];

        test_rendering!(table, MinoT, NormalTypeMino, 2, 2, 5, 5);
    }
}
