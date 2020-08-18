pub struct Mino<K> {
    k: K,
}

pub struct Offset {
    x: Length,
    y: Length,
}

enum Length {
    M2,
    M1,
    No,
    P1,
    P2,
}

type NextMino<T> = (Mino<T>, Offset);

pub trait Minos {
    type RightNext: Minos;
    type LeftNext: Minos;

    fn right(self) -> NextMino<Self::RightNext>;

    fn left(self) -> NextMino<Self::LeftNext>;

    fn cells(&self) -> &[&[u8]];
}

#[macro_export]
macro_rules! mino {
    ( $now:tt, $right:tt, $right_x:tt, $right_y:tt, $left:tt, $left_x:tt, $left_y:tt, $cells:expr ) => {
        impl Mino<$now> {
            pub fn right(self) -> NextMino<$right> {
                next_mino($right, Length::$right_x, Length::$right_y)
            }

            pub fn left(self) -> NextMino<$left> {
                next_mino($left, Length::$left_x, Length::$left_y)
            }

            pub fn cells(&self) -> &[&[u8]] {
                $cells
            }
        }
    };
}

// *o**
pub struct A0;

// *
// o
// *
// *
pub struct A1;

// **o*
pub struct A2;

// *
// *
// o
// *
pub struct A3;

mino!(A0, A1, P1, M1, A3, P1, M2, &[&[1], &[1], &[1], &[1]]);
mino!(A1, A2, M2, P1, A0, M1, P1, &[&[1, 1, 1, 1]]);
mino!(A2, A3, P2, M2, A1, P2, M1, &[&[1], &[1], &[1], &[1]]);
mino!(A3, A0, M1, P2, A2, M2, P2, &[&[1, 1, 1, 1]]);

// **
// **
pub struct B0;

mino!(B0, B0, No, No, B0, No, No, &[&[1, 1], &[1, 1]]);

//  **
// *o
pub struct C0;

// *
// o*
//  *
pub struct C1;

//  o*
// **
pub struct C2;

// *
// *o
//  *
pub struct C3;

mino!(C0, C1, P1, No, C3, No, No, &[&[0, 1, 1], &[1, 1]]);
mino!(C1, C2, M1, P1, C0, M1, No, &[&[1], &[1, 1], &[0, 1]]);
mino!(C2, C3, No, M1, C1, P1, M1, &[&[0, 1, 1], &[1, 1]]);
mino!(C3, C0, No, No, C2, No, P1, &[&[1], &[1, 1], &[0, 1]]);

// **
//  o*
pub struct D0;

//  *
// o*
// *
pub struct D1;

// *o
//  **
pub struct D2;

//  *
// *o
// *
pub struct D3;

mino!(D0, D1, P1, No, D3, No, No, &[&[1, 1], &[0, 1, 1]]);
mino!(D1, D2, M1, P1, D0, M1, No, &[&[0, 1], &[1, 1], &[1]]);
mino!(D2, D3, No, M1, D1, P1, M1, &[&[1, 1], &[0, 1, 1]]);
mino!(D3, D0, No, No, D2, No, P1, &[&[0, 1], &[1, 1], &[1]]);

// *
// *o*
pub struct E0;

// **
// o
// *
pub struct E1;

// *o*
//   *
pub struct E2;

//  *
//  o
// **
pub struct E3;

mino!(E0, E1, P1, No, E3, No, No, &[&[1], &[1, 1, 1]]);
mino!(E1, E2, M1, P1, E0, M1, No, &[&[1], &[1], &[1, 1]]);
mino!(E2, E3, No, M1, E1, P1, M1, &[&[1, 1, 1], &[0, 0, 1]]);
mino!(E3, E0, No, No, E2, No, P1, &[&[0, 1], &[0, 1], &[1, 1]]);

//   *
// *o*
pub struct F0;

// *
// o
// **
pub struct F1;

// *o*
// *
pub struct F2;
// **
//  o
//  *
pub struct F3;

mino!(F0, F1, P1, No, F3, No, No, &[&[0, 0, 1], &[1, 1, 1]]);
mino!(F1, F2, M1, P1, F0, M1, No, &[&[1], &[1], &[1, 1]]);
mino!(F2, F3, No, M1, F1, P1, M1, &[&[1, 1, 1], &[1]]);
mino!(F3, F0, No, No, F2, No, P1, &[&[1, 1], &[0, 1], &[0, 1]]);

//  *
// *o*
pub struct G0;

// *
// o*
// *
pub struct G1;

// *o*
//  *
pub struct G2;

//  *
// *o
//  *
pub struct G3;

mino!(G0, G1, P1, No, G3, No, No, &[&[0, 1, 0], &[1, 1, 1]]);
mino!(G1, G2, M1, P1, G0, M1, No, &[&[1], &[1, 1], &[1]]);
mino!(G2, G3, No, M1, G1, P1, M1, &[&[1, 1, 1], &[1]]);
mino!(G3, G0, No, No, G2, No, P1, &[&[0, 1], &[1, 1], &[0, 1]]);

fn next_mino<S>(next: S, x: Length, y: Length) -> NextMino<S> {
    (Mino { k: next }, Offset { x, y })
}
