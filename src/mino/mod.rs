mod rotate;

use crate::*;

macro_rules! define_minos {
    ( $($element:tt),* $(,)? ) => {
        $(pub struct $element;)*

        pub enum Mino {
            $($element($element),)*
        }

        $(impl From<$element> for Mino {
            fn from(inner: $element) -> Self {
                Self::$element(inner)
            }
        })*

        impl Mino {
            pub fn right(self) -> (Mino, Offset) {
                match self {
                    $(Mino::$element(v) => v.right().into(),)*
                }
            }

            pub fn left(self) -> (Mino, Offset) {
                match self {
                    $(Mino::$element(v) => v.left().into(),)*
                }
            }

            pub fn cells(&self) -> &[&[u8]] {
                match self {
                    $(Mino::$element(v) => v.cells(),)*
                }
            }
        }
    };
}

define_minos!(A0, A1, A2, A3, B0, C0, C1, C2, C3, D0, D1, D2, D3, E0, E1, E2, E3, F0, F1, F2, F3, G0, G1, G2, G3,);
