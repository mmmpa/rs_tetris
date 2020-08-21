use crate::*;

pub struct MinoState<MT: MinoType, MF: MinoForm, Rot: RotationState> {
    _mino: MT,
    _form: MF,
    _state: Rot,
    x: i8,
    y: i8,
}

pub trait NewWithPos {
    fn new_with(x: i8, y: i8) -> Self;
    fn new_with_t(xy: (i8, i8)) -> Self;
}

impl<MT: MinoType, MF: MinoForm, Rot: RotationState> NewWithPos for MinoState<MT, MF, Rot> {
    fn new_with(x: i8, y: i8) -> Self {
        Self {
            _mino: MT::new(),
            _form: MF::new(),
            _state: Rot::new(),
            x,
            y,
        }
    }

    /// for cloning after rotation
    fn new_with_t((x, y): (i8, i8)) -> Self {
        Self::new_with(x, y)
    }
}

/// Provide a mino information for rendering.
pub trait MinoCore: NewWithPos {
    type Mino;
    type Form: MinoForm;
    type Now: RotationState;
    type Right: RotationState;
    type Side: RotationState;
    type Left: RotationState;
    type Cell: CellExe<Mino = Self::Mino, State = Self::Now>;

    /// for cloning
    fn pos(&self) -> (i8, i8);

    /// for moving
    fn offset(&mut self, xy: (i8, i8));

    /// for rendering
    fn mut_with_absolute_cells<F>(&self, f: F)
    where
        F: FnMut(i8, i8);

    /// for hit testing
    fn test_with_absolute_cells<F, T>(&self, f: F) -> bool
    where
        F: Fn(i8, i8) -> bool;
}

macro_rules! define_mino_common {
    () => {
        fn pos(&self) -> (i8, i8) {
            (self.x, self.y)
        }

        fn offset(&mut self, xy: (i8, i8)) {
            self.x += xy.0;
            self.y += xy.1;
        }

        fn mut_with_absolute_cells<F>(&self, mut f: F)
        where
            F: FnMut(i8, i8),
        {
            Self::Cell::cells().iter().for_each(|(x, y)| f(x + self.x, y + self.y));
        }

        fn test_with_absolute_cells<F, T>(&self, f: F) -> bool
        where
            F: Fn(i8, i8) -> bool,
        {
            for (x, y) in Self::Cell::cells().iter() {
                if f(x + self.x, y + self.y) {
                    return true;
                }
            }

            false
        }
    };
}

macro_rules! define_mino_rotation {
    ( $mino_type:tt, $mino_form:tt, $direction:tt, $from:tt => $to:tt ) => {
        impl $direction for MinoState<$mino_type, $mino_form, $from> {
            type Next = MinoState<$mino_type, $mino_form, $to>;
            type Rotation = RotationOffset<$mino_form, $from, $to>;
            type Srs = SrsOffset<$mino_form, $from, $to>;
        }
    };
}

macro_rules! define_mino {
    ( $mino_type:tt, $mino_form:tt ) => {
        impl MinoCore for MinoState<$mino_type, $mino_form, State0> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = State0;
            type Right = StateR;
            type Side = State2;
            type Left = StateL;
            type Cell = Cell<$mino_type, State0>;

            define_mino_common!();
        }

        impl MinoCore for MinoState<$mino_type, $mino_form, StateR> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = StateR;
            type Right = State2;
            type Side = StateL;
            type Left = State0;
            type Cell = Cell<$mino_type, StateR>;

            define_mino_common!();
        }

        impl MinoCore for MinoState<$mino_type, $mino_form, State2> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = State2;
            type Right = StateL;
            type Side = State0;
            type Left = StateR;
            type Cell = Cell<$mino_type, State2>;

            define_mino_common!();
        }

        impl MinoCore for MinoState<$mino_type, $mino_form, StateL> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = StateL;
            type Right = State0;
            type Side = StateR;
            type Left = State2;
            type Cell = Cell<$mino_type, StateL>;

            define_mino_common!();
        }

        define_mino_rotation!($mino_type, $mino_form, Right, State0 => StateR);
        define_mino_rotation!($mino_type, $mino_form, Right, StateR => State2);
        define_mino_rotation!($mino_type, $mino_form, Right, State2 => StateL);
        define_mino_rotation!($mino_type, $mino_form, Right, StateL => State0);

        define_mino_rotation!($mino_type, $mino_form, Left, State0 => StateL);
        define_mino_rotation!($mino_type, $mino_form, Left, StateL => State2);
        define_mino_rotation!($mino_type, $mino_form, Left, State2 => StateR);
        define_mino_rotation!($mino_type, $mino_form, Left, StateR => State0);
    };
}

define_mino!(MinoI, BarTypeMino);
define_mino!(MinoO, NormalTypeMino);
define_mino!(MinoS, NormalTypeMino);
define_mino!(MinoZ, NormalTypeMino);
define_mino!(MinoJ, NormalTypeMino);
define_mino!(MinoL, NormalTypeMino);
define_mino!(MinoT, NormalTypeMino);
