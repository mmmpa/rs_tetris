use crate::*;

pub struct SrsOffset<T: MinoForm, A: RotationState, B: RotationState> {
    t: T,
    a: A,
    b: B,
}

impl<T: MinoForm, A: RotationState, B: RotationState> PlaneNew for SrsOffset<T, A, B> {
    fn plane() -> Self {
        Self {
            t: T::plane(),
            a: A::plane(),
            b: B::plane(),
        }
    }
}

pub trait SrsOffsetExe: PlaneNew {
    type Form: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    fn offset(&self) -> &[(i8, i8)];
}

// Bar

impl SrsOffsetExe for SrsOffset<BarTypeMino, State0, StateR> {
    type Form = BarTypeMino;
    type Now = State0;
    type Next = StateR;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, -1), (0, -2), (-1, -2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, StateR, State0> {
    type Form = BarTypeMino;
    type Now = StateR;
    type Next = State0;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, StateR, State2> {
    type Form = BarTypeMino;
    type Now = StateR;
    type Next = State2;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, State2, StateR> {
    type Form = BarTypeMino;
    type Now = State2;
    type Next = StateR;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, State2, StateL> {
    type Form = BarTypeMino;
    type Now = State2;
    type Next = StateL;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, StateL, State2> {
    type Form = BarTypeMino;
    type Now = StateL;
    type Next = State2;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, StateL, State0> {
    type Form = BarTypeMino;
    type Now = StateL;
    type Next = State0;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<BarTypeMino, State0, StateL> {
    type Form = BarTypeMino;
    type Now = State0;
    type Next = StateL;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
    }
}

// Other than bar

impl SrsOffsetExe for SrsOffset<NormalTypeMino, State0, StateR> {
    type Form = NormalTypeMino;
    type Now = State0;
    type Next = StateR;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, -1), (0, -2), (-1, -2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, StateR, State0> {
    type Form = NormalTypeMino;
    type Now = StateR;
    type Next = State0;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, StateR, State2> {
    type Form = NormalTypeMino;
    type Now = StateR;
    type Next = State2;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, State2, StateR> {
    type Form = NormalTypeMino;
    type Now = State2;
    type Next = StateR;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, State2, StateL> {
    type Form = NormalTypeMino;
    type Now = State2;
    type Next = StateL;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, StateL, State2> {
    type Form = NormalTypeMino;
    type Now = StateL;
    type Next = State2;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, StateL, State0> {
    type Form = NormalTypeMino;
    type Now = StateL;
    type Next = State0;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
    }
}

impl SrsOffsetExe for SrsOffset<NormalTypeMino, State0, StateL> {
    type Form = NormalTypeMino;
    type Now = State0;
    type Next = StateL;

    fn offset(&self) -> &[(i8, i8)] {
        &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
    }
}
