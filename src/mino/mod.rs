mod form;
mod mino;
mod plain_new;
mod rotate;
pub mod rotation;
mod rotation_state;
pub mod srs;

pub use form::*;
pub use plain_new::*;
pub use rotation_state::*;

use crate::*;

macro_rules! define_minos {
    ( $($element:tt),* $(,)? ) => {
        $(pub struct $element;)*

        // Work as a trait that make different minos populate into same array.
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

pub const MINOS: [Mino; 7] = [
    Mino::A0(A0),
    Mino::B0(B0),
    Mino::C0(C0),
    Mino::D0(D0),
    Mino::E0(E0),
    Mino::F0(F0),
    Mino::G0(G0),
];

define_minos!(A0, A1, A2, A3, B0, C0, C1, C2, C3, D0, D1, D2, D3, E0, E1, E2, E3, F0, F1, F2, F3, G0, G1, G2, G3,);
