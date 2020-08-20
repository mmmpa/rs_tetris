use crate::*;

pub trait Right: MinoBase {
    type Next: MinoBase<Form = Self::Form, Now = Self::Right, Right = Self::Side, Side = Self::Left, Left = Self::Now> + Right + Left;
    type Rotation: RotationOffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Right>;
    type Srs: SrsOffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Right>;

    fn right(&self) -> (Self::Next, &[(i8, i8)]) {
        let mut next = Self::Next::new_with_t(self.pos());
        let offset = Self::Rotation::offset();
        next.offset(offset);

        let srs = Self::Srs::offset();

        (next, srs)
    }
}

pub trait Left: MinoBase {
    type Next: MinoBase<Form = Self::Form, Now = Self::Left, Right = Self::Now, Side = Self::Right, Left = Self::Side> + Right + Left;
    type Rotation: RotationOffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Left>;
    type Srs: SrsOffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Left>;

    fn left(&self) -> (Self::Next, &[(i8, i8)]) {
        let mut next = Self::Next::new_with_t(self.pos());
        let offset = Self::Rotation::offset();
        next.offset(offset);

        let srs = Self::Srs::offset();

        (next, srs)
    }
}
pub struct NewState {
    x: i8,
    y: i8,
}

#[cfg(test)]
mod tests {
    use crate::mino::mino::*;
    use crate::{BarTypeMino, State0};
    use std::prelude::v1::*;

    macro_rules! test_rendering {
        ( $table:tt, $mino:tt, $form:tt, $x:tt, $y:tt, $canvas_w:tt, $canvas_h:tt ) => {
            let mut it = $table.iter();

            let mut mino = MinoState::<$mino, $form, State0>::new_with($x, $y);

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

            let mut mino = MinoState::<$mino, $form, State0>::new_with($x, $y);

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

    fn print_test(state: &impl MinoBase, w: usize, h: usize) -> String {
        let mut canvas = vec![vec!["⬜"; w]; h];

        let (x, y) = state.pos();

        state.cells().iter().for_each(|(ox, oy)| {
            canvas[(y + oy) as usize][(x + ox) as usize] = "⬛";
        });

        canvas.into_iter().flat_map(|s| s).collect::<Vec<_>>().join("")
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

        test_rendering!(table, A, BarTypeMino, 2, 3, 7, 7);
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

        test_rendering!(table, C, NormalTypeMino, 1, 1, 5, 5);
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

        test_rendering!(table, D, NormalTypeMino, 1, 1, 5, 5);
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

        test_rendering!(table, E, NormalTypeMino, 1, 1, 5, 5);
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

        test_rendering!(table, F, NormalTypeMino, 1, 1, 5, 5);
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

        test_rendering!(table, G, NormalTypeMino, 1, 1, 5, 5);
    }
}
