use crate::*;
use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct MinoState<MT: MinoType, Rot: RotationState> {
    _mino: MT,
    _state: Rot,
    pub x: i8,
    pub y: i8,
}

impl<MT: MinoType, Rot: RotationState> MinoState<MT, Rot> {
    pub fn new(x: i8, y: i8) -> Self {
        Self {
            _mino: MT::new(),
            _state: Rot::new(),
            x,
            y,
        }
    }
}

use crate::*;

pub trait MinoFn:
    NewWith + Right + Left + WithCell + Rotatable + IsState + Into<MinoAggregation>
{
}

impl<T: NewWith + Right + Left + WithCell + Rotatable + IsState + Into<MinoAggregation>> MinoFn
    for T
{
}

/// Provide a mino information for rendering.
pub trait MinoCore {
    type Form: MinoForm;
    type Now: RotationState;
    type Right: RotationState;
    type Side: RotationState;
    type Left: RotationState;
}

macro_rules! define_mino {
    ( $mino_type:tt, $mino_form:tt ) => {
        impl MinoCore for MinoState<$mino_type, State0> {
            type Form = $mino_form;
            type Now = State0;
            type Right = StateR;
            type Side = State2;
            type Left = StateL;
        }

        impl MinoCore for MinoState<$mino_type, StateR> {
            type Form = $mino_form;
            type Now = StateR;
            type Right = State2;
            type Side = StateL;
            type Left = State0;
        }

        impl MinoCore for MinoState<$mino_type, StateL> {
            type Form = $mino_form;
            type Now = StateL;
            type Right = State0;
            type Side = StateR;
            type Left = State2;
        }

        impl MinoCore for MinoState<$mino_type, State2> {
            type Form = $mino_form;
            type Now = State2;
            type Right = StateL;
            type Side = State0;
            type Left = StateR;
        }

        define_rotation!(Right, $mino_type, $mino_form, State0 => StateR);
        define_rotation!(Right, $mino_type, $mino_form, StateR => State2);
        define_rotation!(Right, $mino_type, $mino_form, State2 => StateL);
        define_rotation!(Right, $mino_type, $mino_form, StateL => State0);
        define_rotation!(Left, $mino_type, $mino_form, State0 => StateL);
        define_rotation!(Left, $mino_type, $mino_form, StateL => State2);
        define_rotation!(Left, $mino_type, $mino_form, State2 => StateR);
        define_rotation!(Left, $mino_type, $mino_form, StateR => State0);
    };
}

define_mino!(MinoI, BarTypeMino);
define_mino!(MinoO, NormalTypeMino);
define_mino!(MinoS, NormalTypeMino);
define_mino!(MinoZ, NormalTypeMino);
define_mino!(MinoJ, NormalTypeMino);
define_mino!(MinoL, NormalTypeMino);
define_mino!(MinoT, NormalTypeMino);

define_mino_aggregation!(
    Is0 => MinoState<MinoI, State0>,
    Os0 => MinoState<MinoO, State0>,
    Ss0 => MinoState<MinoS, State0>,
    Zs0 => MinoState<MinoZ, State0>,
    Js0 => MinoState<MinoJ, State0>,
    Ls0 => MinoState<MinoL, State0>,
    Ts0 => MinoState<MinoT, State0>,
    IsR => MinoState<MinoI, StateR>,
    OsR => MinoState<MinoO, StateR>,
    SsR => MinoState<MinoS, StateR>,
    ZsR => MinoState<MinoZ, StateR>,
    JsR => MinoState<MinoJ, StateR>,
    LsR => MinoState<MinoL, StateR>,
    TsR => MinoState<MinoT, StateR>,
    Is2 => MinoState<MinoI, State2>,
    Os2 => MinoState<MinoO, State2>,
    Ss2 => MinoState<MinoS, State2>,
    Zs2 => MinoState<MinoZ, State2>,
    Js2 => MinoState<MinoJ, State2>,
    Ls2 => MinoState<MinoL, State2>,
    Ts2 => MinoState<MinoT, State2>,
    IsL => MinoState<MinoI, StateL>,
    OsL => MinoState<MinoO, StateL>,
    SsL => MinoState<MinoS, StateL>,
    ZsL => MinoState<MinoZ, StateL>,
    JsL => MinoState<MinoJ, StateL>,
    LsL => MinoState<MinoL, StateL>,
    TsL => MinoState<MinoT, StateL>,
);

define_first_minos!(
    Is0 => MinoState<MinoI, State0>,
    Os0 => MinoState<MinoO, State0>,
    Ss0 => MinoState<MinoS, State0>,
    Zs0 => MinoState<MinoZ, State0>,
    Js0 => MinoState<MinoJ, State0>,
    Ls0 => MinoState<MinoL, State0>,
    Ts0 => MinoState<MinoT, State0>,
);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let size = std::mem::size_of::<MinoState<MinoO, State0>>();
        assert_eq!(2, size);
    }
}
