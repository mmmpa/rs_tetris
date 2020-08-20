#[macro_use]
mod macros;

mod cells;
mod form;
mod mino;
mod mino_base;
mod plain_new;
mod rotation_offset;
mod rotation_state;
mod srs_offset;

pub use cells::*;
pub use form::*;
pub use mino::*;
pub use mino_base::*;
pub use plain_new::*;
pub use rotation_offset::*;
pub use rotation_state::*;
pub use srs_offset::*;

use crate::*;

// pub const MINOS: [Mino; 7] = [
//     Mino::A0(A0),
//     Mino::B0(B0),
//     Mino::C0(C0),
//     Mino::D0(D0),
//     Mino::E0(E0),
//     Mino::F0(F0),
//     Mino::G0(G0),
// ];
