use crate::*;

pub struct RotationOffset<T: MinoForm, A: RotationState, B: RotationState>(T, A, B);

pub trait RotationOffsetExe {
    type Form: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    fn offset() -> (i8, i8);
}

// Bar

impl RotationOffsetExe for RotationOffset<BarTypeMino, State0, StateR> {
    type Form = BarTypeMino;
    type Now = State0;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (1, -1)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, StateR, State0> {
    type Form = BarTypeMino;
    type Now = StateR;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (-1, 1)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, StateR, State2> {
    type Form = BarTypeMino;
    type Now = StateR;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (-2, 1)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, State2, StateR> {
    type Form = BarTypeMino;
    type Now = State2;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (2, -1)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, State2, StateL> {
    type Form = BarTypeMino;
    type Now = State2;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (2, -2)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, StateL, State2> {
    type Form = BarTypeMino;
    type Now = StateL;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (-2, 2)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, StateL, State0> {
    type Form = BarTypeMino;
    type Now = StateL;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (-1, 2)
    }
}

impl RotationOffsetExe for RotationOffset<BarTypeMino, State0, StateL> {
    type Form = BarTypeMino;
    type Now = State0;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (1, -2)
    }
}

// Other than bar

impl RotationOffsetExe for RotationOffset<NormalTypeMino, State0, StateR> {
    type Form = NormalTypeMino;
    type Now = State0;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (1, 0)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, StateR, State0> {
    type Form = NormalTypeMino;
    type Now = StateR;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (-1, 0)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, StateR, State2> {
    type Form = NormalTypeMino;
    type Now = StateR;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (-1, 1)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, State2, StateR> {
    type Form = NormalTypeMino;
    type Now = State2;
    type Next = StateR;

    fn offset() -> (i8, i8) {
        (1, -1)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, State2, StateL> {
    type Form = NormalTypeMino;
    type Now = State2;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (0, -1)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, StateL, State2> {
    type Form = NormalTypeMino;
    type Now = StateL;
    type Next = State2;

    fn offset() -> (i8, i8) {
        (0, 1)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, StateL, State0> {
    type Form = NormalTypeMino;
    type Now = StateL;
    type Next = State0;

    fn offset() -> (i8, i8) {
        (0, 0)
    }
}

impl RotationOffsetExe for RotationOffset<NormalTypeMino, State0, StateL> {
    type Form = NormalTypeMino;
    type Now = State0;
    type Next = StateL;

    fn offset() -> (i8, i8) {
        (0, 0)
    }
}
