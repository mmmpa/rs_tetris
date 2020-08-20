use crate::*;

pub trait MinoBase: NewWithPos {
    type Mino;
    type Form: MinoForm;
    type Now: RotationState;
    type Right: RotationState;
    type Side: RotationState;
    type Left: RotationState;
    type Cell: CellExt<Mino = Self::Mino, State = Self::Now>;

    fn pos(&self) -> (i8, i8);
    fn offset(&mut self, xy: (i8, i8));

    fn cells(&self) -> &'static [(i8, i8)] {
        Self::Cell::cells()
    }
}

pub trait NewWithPos {
    fn new_with(x: i8, y: i8) -> Self;
    fn new_with_t(xy: (i8, i8)) -> Self;
}

impl<MT: PlaneNew, MF: MinoForm, Rot: RotationState> NewWithPos for MinoState<MT, MF, Rot> {
    fn new_with(x: i8, y: i8) -> Self {
        Self {
            mino: MT::plane(),
            form: MF::plane(),
            state: Rot::plane(),
            x,
            y,
        }
    }

    fn new_with_t((x, y): (i8, i8)) -> Self {
        Self::new_with(x, y)
    }
}

pub struct MinoState<MT, MF: MinoForm, Rot: RotationState> {
    mino: MT,
    form: MF,
    state: Rot,
    x: i8,
    y: i8,
}

macro_rules! define_from {
    ( $mino:tt, $form:tt ) => {
        impl From<$mino> for MinoState<$mino, $form, State0> {
            fn from(_: $mino) -> Self {
                Self::new_with(0, 0)
            }
        }
    };
}

define_from!(A, BarTypeMino);
define_from!(B, NormalTypeMino);
define_from!(C, NormalTypeMino);
define_from!(D, NormalTypeMino);
define_from!(E, NormalTypeMino);
define_from!(F, NormalTypeMino);
define_from!(G, NormalTypeMino);

macro_rules! define_mino {
    ( $mino_type:tt, $mino_form:tt ) => {
        impl MinoBase for MinoState<$mino_type, $mino_form, State0> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = State0;
            type Right = StateR;
            type Side = State2;
            type Left = StateL;
            type Cell = Cell<$mino_type, State0>;

            fn pos(&self) -> (i8, i8) {
                (self.x, self.y)
            }
            fn offset(&mut self, xy: (i8, i8)) {
                self.x += xy.0;
                self.y += xy.1;
            }
        }

        impl MinoBase for MinoState<$mino_type, $mino_form, StateR> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = StateR;
            type Right = State2;
            type Side = StateL;
            type Left = State0;
            type Cell = Cell<$mino_type, StateR>;

            fn pos(&self) -> (i8, i8) {
                (self.x, self.y)
            }
            fn offset(&mut self, xy: (i8, i8)) {
                self.x += xy.0;
                self.y += xy.1;
            }
        }

        impl MinoBase for MinoState<$mino_type, $mino_form, State2> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = State2;
            type Right = StateL;
            type Side = State0;
            type Left = StateR;
            type Cell = Cell<$mino_type, State2>;

            fn pos(&self) -> (i8, i8) {
                (self.x, self.y)
            }
            fn offset(&mut self, xy: (i8, i8)) {
                self.x += xy.0;
                self.y += xy.1;
            }
        }

        impl MinoBase for MinoState<$mino_type, $mino_form, StateL> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = StateL;
            type Right = State0;
            type Side = StateR;
            type Left = State2;
            type Cell = Cell<$mino_type, StateL>;

            fn pos(&self) -> (i8, i8) {
                (self.x, self.y)
            }
            fn offset(&mut self, xy: (i8, i8)) {
                self.x += xy.0;
                self.y += xy.1;
            }
        }

        impl Right for MinoState<$mino_type, $mino_form, State0> {
            type Next = MinoState<$mino_type, $mino_form, StateR>;
            type Rotation = RotationOffset<$mino_form, State0, StateR>;
            type Srs = SrsOffset<$mino_form, State0, StateR>;
        }

        impl Right for MinoState<$mino_type, $mino_form, StateR> {
            type Next = MinoState<$mino_type, $mino_form, State2>;
            type Rotation = RotationOffset<$mino_form, StateR, State2>;
            type Srs = SrsOffset<$mino_form, StateR, State2>;
        }

        impl Right for MinoState<$mino_type, $mino_form, State2> {
            type Next = MinoState<$mino_type, $mino_form, StateL>;
            type Rotation = RotationOffset<$mino_form, State2, StateL>;
            type Srs = SrsOffset<$mino_form, State2, StateL>;
        }

        impl Right for MinoState<$mino_type, $mino_form, StateL> {
            type Next = MinoState<$mino_type, $mino_form, State0>;
            type Rotation = RotationOffset<$mino_form, StateL, State0>;
            type Srs = SrsOffset<$mino_form, StateL, State0>;
        }

        impl Left for MinoState<$mino_type, $mino_form, State0> {
            type Next = MinoState<$mino_type, $mino_form, StateL>;
            type Rotation = RotationOffset<$mino_form, State0, StateL>;
            type Srs = SrsOffset<$mino_form, State0, StateL>;
        }

        impl Left for MinoState<$mino_type, $mino_form, StateL> {
            type Next = MinoState<$mino_type, $mino_form, State2>;
            type Rotation = RotationOffset<$mino_form, StateL, State2>;
            type Srs = SrsOffset<$mino_form, StateL, State2>;
        }

        impl Left for MinoState<$mino_type, $mino_form, State2> {
            type Next = MinoState<$mino_type, $mino_form, StateR>;
            type Rotation = RotationOffset<$mino_form, State2, StateR>;
            type Srs = SrsOffset<$mino_form, State2, StateR>;
        }

        impl Left for MinoState<$mino_type, $mino_form, StateR> {
            type Next = MinoState<$mino_type, $mino_form, State0>;
            type Rotation = RotationOffset<$mino_form, StateR, State0>;
            type Srs = SrsOffset<$mino_form, StateR, State0>;
        }
    };
}

define_markers!(A, B, C, D, E, F, G);

define_mino!(A, BarTypeMino);
define_mino!(B, NormalTypeMino);
define_mino!(C, NormalTypeMino);
define_mino!(D, NormalTypeMino);
define_mino!(E, NormalTypeMino);
define_mino!(F, NormalTypeMino);
define_mino!(G, NormalTypeMino);
