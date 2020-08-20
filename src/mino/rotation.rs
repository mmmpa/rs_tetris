use crate::*;

pub struct Offset<T: MinoForm, A: RotationState, B: RotationState> {
    t: T,
    a: A,
    b: B,
}

pub trait OffsetExe {
    type Form: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    fn offset() -> (i8, i8);
}

// Bar

impl OffsetExe for Offset<BarTypeMino, State0, StateR> {
    type Form = BarTypeMino;
    type Now = State0;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (1, -1)
    }
}

impl OffsetExe for Offset<BarTypeMino, StateR, State0> {
    type Form = BarTypeMino;
    type Now = StateR;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (-1, 1)
    }
}

impl OffsetExe for Offset<BarTypeMino, StateR, State2> {
    type Form = BarTypeMino;
    type Now = StateR;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (-2, 1)
    }
}

impl OffsetExe for Offset<BarTypeMino, State2, StateR> {
    type Form = BarTypeMino;
    type Now = State2;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (2, -1)
    }
}

impl OffsetExe for Offset<BarTypeMino, State2, StateL> {
    type Form = BarTypeMino;
    type Now = State2;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (2, -2)
    }
}

impl OffsetExe for Offset<BarTypeMino, StateL, State2> {
    type Form = BarTypeMino;
    type Now = StateL;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (-2, 2)
    }
}

impl OffsetExe for Offset<BarTypeMino, StateL, State0> {
    type Form = BarTypeMino;
    type Now = StateL;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (-1, 2)
    }
}

impl OffsetExe for Offset<BarTypeMino, State0, StateL> {
    type Form = BarTypeMino;
    type Now = State0;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (1, -2)
    }
}

// Other than bar

impl OffsetExe for Offset<NormalTypeMino, State0, StateR> {
    type Form = NormalTypeMino;
    type Now = State0;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (1, 0)
    }
}

impl OffsetExe for Offset<NormalTypeMino, StateR, State0> {
    type Form = NormalTypeMino;
    type Now = StateR;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (-1, 0)
    }
}

impl OffsetExe for Offset<NormalTypeMino, StateR, State2> {
    type Form = NormalTypeMino;
    type Now = StateR;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (-1, 1)
    }
}

impl OffsetExe for Offset<NormalTypeMino, State2, StateR> {
    type Form = NormalTypeMino;
    type Now = State2;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (1, -1)
    }
}

impl OffsetExe for Offset<NormalTypeMino, State2, StateL> {
    type Form = NormalTypeMino;
    type Now = State2;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (0, -1)
    }
}

impl OffsetExe for Offset<NormalTypeMino, StateL, State2> {
    type Form = NormalTypeMino;
    type Now = StateL;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (0, 1)
    }
}

impl OffsetExe for Offset<NormalTypeMino, StateL, State0> {
    type Form = NormalTypeMino;
    type Now = StateL;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (0, 0)
    }
}

impl OffsetExe for Offset<NormalTypeMino, State0, StateL> {
    type Form = NormalTypeMino;
    type Now = State0;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (0, 0)
    }
}
