use crate::*;

pub struct Cell<A, Rot: RotationState>(A, Rot);

pub trait CellExe {
    type Mino;
    type State;

    fn cells() -> &'static [(i8, i8)];
}

macro_rules! define_cells {
    ( $mino:tt, $state:tt, $cells:expr ) => {
        /// Associated types are just for binding types in other trait definition.
        impl CellExe for Cell<$mino, $state> {
            type Mino = $mino;
            type State = $state;

            fn cells() -> &'static [(i8, i8)] {
                &$cells
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
define_cells!(MinoI, State0, [(0, 0), (1, 0), (2, 0), (3, 0)]);
define_cells!(MinoI, StateR, [(0, 3), (0, 2), (0, 1), (0, 0)]);
define_cells!(MinoI, State2, [(0, 0), (1, 0), (2, 0), (3, 0)]);
define_cells!(MinoI, StateL, [(0, 3), (0, 2), (0, 1), (0, 0)]);

// ⬛⬛
// ⬛⬛
define_cells!(MinoO, State0, [(1, 1), (0, 1), (1, 0), (0, 0)]);
define_cells!(MinoO, StateR, [(1, 1), (0, 1), (1, 0), (0, 0)]);
define_cells!(MinoO, State2, [(1, 1), (0, 1), (1, 0), (0, 0)]);
define_cells!(MinoO, StateL, [(1, 1), (0, 1), (1, 0), (0, 0)]);

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
define_cells!(MinoS, State0, [(0, 1), (1, 1), (1, 0), (2, 0)]);
define_cells!(MinoS, StateR, [(1, 2), (0, 1), (1, 1), (0, 0)]);
define_cells!(MinoS, State2, [(0, 1), (1, 1), (1, 0), (2, 0)]);
define_cells!(MinoS, StateL, [(1, 2), (0, 1), (1, 1), (0, 0)]);

// ⬛⬛
// 　⬜⬛
// -----
// 　⬛
// ⬜⬛
// ⬛
// -----
// ⬛⬜
// 　⬛⬛
// -----
// 　⬛
// ⬛⬜
// ⬛
define_cells!(MinoZ, State0, [(0, 0), (1, 0), (1, 1), (2, 1)]);
define_cells!(MinoZ, StateR, [(1, 0), (0, 1), (1, 1), (0, 2)]);
define_cells!(MinoZ, State2, [(0, 0), (1, 0), (1, 1), (2, 1)]);
define_cells!(MinoZ, StateL, [(1, 0), (0, 1), (1, 1), (0, 2)]);

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
define_cells!(MinoJ, State0, [(0, 1), (1, 1), (2, 1), (0, 0)]);
define_cells!(MinoJ, StateR, [(0, 2), (0, 1), (1, 0), (0, 0)]);
define_cells!(MinoJ, State2, [(2, 1), (0, 0), (1, 0), (2, 0)]);
define_cells!(MinoJ, StateL, [(1, 2), (0, 2), (1, 1), (1, 0),]);

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
define_cells!(MinoL, State0, [(0, 1), (1, 1), (2, 1), (2, 0)]);
define_cells!(MinoL, StateR, [(0, 2), (1, 2), (0, 0), (0, 1)]);
define_cells!(MinoL, State2, [(0, 1), (0, 0), (1, 0), (2, 0)]);
define_cells!(MinoL, StateL, [(1, 2), (1, 1), (0, 0), (1, 0)]);

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
define_cells!(MinoT, State0, [(0, 1), (1, 1), (2, 1), (1, 0)]);
define_cells!(MinoT, StateR, [(0, 2), (0, 1), (1, 1), (0, 0)]);
define_cells!(MinoT, State2, [(1, 1), (0, 0), (1, 0), (2, 0)]);
define_cells!(MinoT, StateL, [(1, 2), (1, 1), (0, 1), (1, 0)]);
