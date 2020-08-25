use crate::*;

pub trait Cell {
    fn cells() -> &'static [(i8, i8)];
}

macro_rules! define_cells {
    ( $mino:tt, $state:tt, $cells:expr ) => {
        impl<T: MinoForm> Cell for MinoState<$mino, T, $state> {
            fn cells() -> &'static [(i8, i8)] {
                &$cells
            }
        }
    };
}

// ⬛⬜⬛⬛
// -----
// 　⬛
// ⬜⬛
// 　⬛
// 　⬛
// -----
// 　⬜
// ⬛⬛⬛⬛
// -----
// ⬛
// ⬜
// ⬛
// ⬛
define_cells!(MinoI, State0, [(-1, 0), (0, 0), (1, 0), (2, 0)]);
define_cells!(MinoI, StateR, [(1, 2), (1, 1), (1, 0), (1, -1)]);
define_cells!(MinoI, State2, [(-1, 1), (0, 1), (1, 1), (2, 1)]);
define_cells!(MinoI, StateL, [(0, 2), (0, 1), (0, 0), (0, -1)]);

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
define_cells!(MinoS, State0, [(-1, 0), (0, 0), (1, -1), (0, -1)]);
define_cells!(MinoS, StateR, [(1, 1), (0, 0), (1, 0), (0, -1)]);
define_cells!(MinoS, State2, [(-1, 1), (0, 1), (1, 0), (0, 0)]);
define_cells!(MinoS, StateL, [(0, 1), (-1, 0), (0, 0), (-1, -1)]);

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
define_cells!(MinoZ, State0, [(0, 0), (1, 0), (-1, -1), (0, -1)]);
define_cells!(MinoZ, StateR, [(0, 1), (1, 0), (0, 0), (1, -1)]);
define_cells!(MinoZ, State2, [(0, 1), (1, 1), (-1, 0), (0, 0)]);
define_cells!(MinoZ, StateL, [(-1, 1), (0, 0), (-1, 0), (0, -1)]);

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
define_cells!(MinoJ, State0, [(-1, 0), (0, 0), (1, 0), (-1, -1)]);
define_cells!(MinoJ, StateR, [(0, 1), (0, 0), (1, -1), (0, -1)]);
define_cells!(MinoJ, State2, [(1, 1), (-1, 0), (0, 0), (1, 0)]);
define_cells!(MinoJ, StateL, [(-1, 1), (0, 1), (0, 0), (0, -1)]);

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
define_cells!(MinoL, State0, [(-1, 0), (0, 0), (1, 0), (1, -1)]);
define_cells!(MinoL, StateR, [(1, 1), (0, 1), (0, 0), (0, -1)]);
define_cells!(MinoL, State2, [(-1, 1), (1, 0), (0, 0), (-1, 0)]);
define_cells!(MinoL, StateL, [(0, 1), (0, 0), (0, -1), (-1, -1)]);

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
define_cells!(MinoT, State0, [(-1, 0), (0, 0), (1, 0), (0, -1)]);
define_cells!(MinoT, StateR, [(0, 1), (0, 0), (1, 0), (0, -1)]);
define_cells!(MinoT, State2, [(0, 1), (-1, 0), (1, 0), (0, 0)]);
define_cells!(MinoT, StateL, [(0, 1), (-1, 0), (0, 0), (0, -1)]);
