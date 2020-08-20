mod offset;
mod offset_type;

pub(crate) use offset::*;
pub use offset_type::*;

use crate::*;

pub struct Srs<T: MinoForm, A: RotationState> {
    t: T,
    rot: A,
}

impl<T: MinoForm, A: RotationState> PlaneNew for Srs<T, A> {
    fn plane() -> Self {
        Self {
            t: T::plane(),
            rot: A::plane(),
        }
    }
}

pub trait SrsRight {
    type Type: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    type NextSrs: SrsLeft + SrsRight + PlaneNew;
    type SrsOffset: SrsOffsetExe;

    fn right_srs(&self) -> (Self::NextSrs, Self::SrsOffset) {
        let next = Self::NextSrs::plane();
        let offset = Self::SrsOffset::plane();

        (next, offset)
    }
}

pub trait SrsLeft {
    type Type: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    type NextSrs: SrsLeft + SrsRight + PlaneNew;
    type SrsOffset: SrsOffsetExe;

    fn left_srs(&self) -> (Self::NextSrs, Self::SrsOffset) {
        let next = Self::NextSrs::plane();
        let offset = Self::SrsOffset::plane();

        (next, offset)
    }
}

impl SrsRight for Srs<NormalTypeMino, State0> {
    type Type = NormalTypeMino;
    type Now = State0;
    type Next = StateR;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<NormalTypeMino, State0> {
    type Type = NormalTypeMino;
    type Now = State0;
    type Next = StateL;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<NormalTypeMino, StateR> {
    type Type = NormalTypeMino;
    type Now = StateR;
    type Next = State2;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<NormalTypeMino, StateR> {
    type Type = NormalTypeMino;
    type Now = StateR;
    type Next = State0;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<NormalTypeMino, StateL> {
    type Type = NormalTypeMino;
    type Now = StateL;
    type Next = State0;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<NormalTypeMino, StateL> {
    type Type = NormalTypeMino;
    type Now = StateL;
    type Next = State2;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<NormalTypeMino, State2> {
    type Type = NormalTypeMino;
    type Now = State2;
    type Next = StateL;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<NormalTypeMino, State2> {
    type Type = NormalTypeMino;
    type Now = State2;
    type Next = StateR;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<BarTypeMino, State0> {
    type Type = BarTypeMino;
    type Now = State0;
    type Next = StateR;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<BarTypeMino, State0> {
    type Type = BarTypeMino;
    type Now = State0;
    type Next = StateL;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<BarTypeMino, StateR> {
    type Type = BarTypeMino;
    type Now = StateR;
    type Next = State2;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<BarTypeMino, StateR> {
    type Type = BarTypeMino;
    type Now = StateR;
    type Next = State0;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<BarTypeMino, StateL> {
    type Type = BarTypeMino;
    type Now = StateL;
    type Next = State0;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<BarTypeMino, StateL> {
    type Type = BarTypeMino;
    type Now = StateL;
    type Next = State2;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsRight for Srs<BarTypeMino, State2> {
    type Type = BarTypeMino;
    type Now = State2;
    type Next = StateL;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

impl SrsLeft for Srs<BarTypeMino, State2> {
    type Type = BarTypeMino;
    type Now = State2;
    type Next = StateR;

    type NextSrs = Srs<Self::Type, Self::Next>;
    type SrsOffset = SrsOffset<Self::Type, Self::Now, Self::Next>;
}

#[cfg(test)]
mod tests {
    use crate::mino::srs::{BarTypeMino, Srs, SrsOffset, State0, StateR};
    #[test]
    fn test() {
        //use core::mem::{size_of, size_of_val};
        //
        //let srs_size = size_of::<SrsOffset<BarTypeMino, State0, StateR>>(); // == 0
        //assert_eq!(srs_size, 0);
        //
        //let mino_size = size_of::<Srs<State0>>(); // == 0
        //assert_eq!(mino_size, 0);

        // let srs = SrsOffset::new(NotBar, Rot0, RotR);
        // assert_eq!(size_of_val(&srs), 0);
    }
}
