use crate::*;

pub trait MinoBase: PlaneNew {
    type Form: MinoForm;
    type Now: RotationState;
    type Right: RotationState;
    type Side: RotationState;
    type Left: RotationState;
}

pub struct NewMino<MT: PlaneNew, MF: MinoForm, Rot: RotationState> {
    m: MT,
    t: MF,
    rot: Rot,
}

impl<MT: PlaneNew, MF: MinoForm, Rot: RotationState> PlaneNew for NewMino<MT, MF, Rot> {
    fn plane() -> Self {
        Self {
            m: MT::plane(),
            t: MF::plane(),
            rot: Rot::plane(),
        }
    }
}
macro_rules! define_mino {
    ( $mino_type:tt, $mino_form:tt ) => {
        impl MinoBase for NewMino<$mino_type, $mino_form, State0> {
            type Form = $mino_form;
            type Now = State0;
            type Right = StateR;
            type Side = State2;
            type Left = StateL;
        }

        impl MinoBase for NewMino<$mino_type, $mino_form, StateR> {
            type Form = $mino_form;
            type Now = StateR;
            type Right = State2;
            type Side = StateL;
            type Left = State0;
        }

        impl MinoBase for NewMino<$mino_type, $mino_form, State2> {
            type Form = $mino_form;
            type Now = State2;
            type Right = StateL;
            type Side = State0;
            type Left = StateR;
        }

        impl MinoBase for NewMino<$mino_type, $mino_form, StateL> {
            type Form = $mino_form;
            type Now = StateL;
            type Right = State0;
            type Side = StateR;
            type Left = State2;
        }

        impl Right for NewMino<$mino_type, $mino_form, State0> {
            type Next = NewMino<$mino_type, $mino_form, StateR>;
            type Srs = srs::Offset<$mino_form, State0, StateR>;
        }

        impl Right for NewMino<$mino_type, $mino_form, StateR> {
            type Next = NewMino<$mino_type, $mino_form, State2>;
            type Srs = srs::Offset<$mino_form, StateR, State2>;
        }

        impl Right for NewMino<$mino_type, $mino_form, State2> {
            type Next = NewMino<$mino_type, $mino_form, StateL>;
            type Srs = srs::Offset<$mino_form, State2, StateL>;
        }

        impl Right for NewMino<$mino_type, $mino_form, StateL> {
            type Next = NewMino<$mino_type, $mino_form, State0>;
            type Srs = srs::Offset<$mino_form, StateL, State0>;
        }

        impl Left for NewMino<$mino_type, $mino_form, State0> {
            type Next = NewMino<$mino_type, $mino_form, StateL>;
            type Srs = srs::Offset<$mino_form, State0, StateL>;
        }

        impl Left for NewMino<$mino_type, $mino_form, StateL> {
            type Next = NewMino<$mino_type, $mino_form, State2>;
            type Srs = srs::Offset<$mino_form, StateL, State2>;
        }

        impl Left for NewMino<$mino_type, $mino_form, State2> {
            type Next = NewMino<$mino_type, $mino_form, StateR>;
            type Srs = srs::Offset<$mino_form, State2, StateR>;
        }

        impl Left for NewMino<$mino_type, $mino_form, StateR> {
            type Next = NewMino<$mino_type, $mino_form, State0>;
            type Srs = srs::Offset<$mino_form, StateR, State0>;
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

pub trait Right: MinoBase {
    type Next: MinoBase<Form = Self::Form, Now = Self::Right, Right = Self::Side, Side = Self::Left, Left = Self::Now> + Right + Left;
    type Srs: srs::OffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Right>;

    fn right(&self) -> Self::Next {
        Self::Next::plane()
    }
}

pub trait Left: MinoBase {
    type Next: MinoBase<Form = Self::Form, Now = Self::Left, Right = Self::Now, Side = Self::Right, Left = Self::Side> + Right + Left;
    type Srs: srs::OffsetExe<Form = Self::Form, Now = Self::Now, Next = Self::Left>;

    fn left(&self) -> Self::Next {
        Self::Next::plane()
    }
}

#[cfg(test)]
mod tests {
    use crate::mino::mino::*;
    use crate::{BarTypeMino, State0};

    #[test]
    fn test() {
        let m = NewMino {
            m: A,
            t: BarTypeMino,
            rot: State0,
        };

        let m = m.left();
        let m = m.left();
        let m = m.left();
        let m = m.left();
        let m = m.right();
        let m = m.right();
        let m = m.right();
        let m = m.right();
    }
}
