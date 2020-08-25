use crate::*;
use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct MinoState<MT: MinoType, Rot: RotationState> {
    _mino: MT,
    _state: Rot,
    x: i8,
    y: i8,
}

pub trait NewWithPos {
    fn new_with(x: i8, y: i8) -> Self;
    fn new_with_t(xy: (i8, i8)) -> Self;
}

impl<MT: MinoType, Rot: RotationState> NewWithPos for MinoState<MT, Rot> {
    fn new_with(x: i8, y: i8) -> Self {
        Self {
            _mino: MT::new(),
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

pub trait MinoFn:
    NewWithPos + Right + Left + Cell + AbsoluteCell + Rotatable + Is + Into<Minos>
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

pub trait Pos {
    /// for cloning
    fn pos(&self) -> (i8, i8);

    /// for moving and rotation
    fn absolute(&mut self, xy: (i8, i8));
    fn offset(&mut self, xy: (i8, i8));
}

impl<MT: MinoType, Rot: RotationState> Pos for MinoState<MT, Rot> {
    fn pos(&self) -> (i8, i8) {
        (self.x, self.y)
    }

    fn absolute(&mut self, xy: (i8, i8)) {
        self.x = xy.0;
        self.y = xy.1;
    }

    fn offset(&mut self, xy: (i8, i8)) {
        self.x += xy.0;
        self.y += xy.1;
    }
}

pub trait Rotatable {
    fn is_rotatable(&self) -> bool {
        true
    }
}

#[rustfmt::skip]
impl<Rot: RotationState> Rotatable for MinoState<MinoO, Rot> { fn is_rotatable(&self) -> bool { false } }
impl<Rot: RotationState> Rotatable for MinoState<MinoI, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoS, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoZ, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoJ, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoL, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoT, Rot> {}

pub trait Is {
    fn is_0(&self) -> bool;
    fn is_r(&self) -> bool;
    fn is_l(&self) -> bool;
    fn is_2(&self) -> bool;
}

#[rustfmt::skip]
impl<MT: MinoType> Is for MinoState<MT, State0> {
    fn is_0(&self) -> bool { true }
    fn is_r(&self) -> bool { false }
    fn is_l(&self) -> bool { false }
    fn is_2(&self) -> bool { false }
}

#[rustfmt::skip]
impl<MT: MinoType> Is for MinoState<MT, StateR> {
    fn is_0(&self) -> bool { false }
    fn is_r(&self) -> bool { true }
    fn is_l(&self) -> bool { false }
    fn is_2(&self) -> bool { false }
}

#[rustfmt::skip]
impl<MT: MinoType> Is for MinoState<MT, StateL> {
    fn is_0(&self) -> bool { false }
    fn is_r(&self) -> bool { false }
    fn is_l(&self) -> bool { true }
    fn is_2(&self) -> bool { false }
}

#[rustfmt::skip]
impl<MT: MinoType> Is for MinoState<MT, State2> {
    fn is_0(&self) -> bool { false }
    fn is_r(&self) -> bool { false }
    fn is_l(&self) -> bool { false }
    fn is_2(&self) -> bool { true }
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

        impl MinoFn for MinoState<$mino_type, State0> {}
        impl MinoFn for MinoState<$mino_type, StateR> {}
        impl MinoFn for MinoState<$mino_type, State2> {}
        impl MinoFn for MinoState<$mino_type, StateL> {}
 };
}

define_mino!(MinoI, BarTypeMino);
define_mino!(MinoO, NormalTypeMino);
define_mino!(MinoS, NormalTypeMino);
define_mino!(MinoZ, NormalTypeMino);
define_mino!(MinoJ, NormalTypeMino);
define_mino!(MinoL, NormalTypeMino);
define_mino!(MinoT, NormalTypeMino);

macro_rules! define_minos {
    ( $( $name:tt => $state:path ),* $(,)? ) => {
        #[derive(Debug,  Copy, Clone)]
        pub enum Minos {
            $( $name($state), )*
        }
        $(
            impl Into<Minos> for $state {
                fn into(self) -> Minos {
                    Minos::$name(self)
                }
            }
        )*
    }
}

define_minos!(
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

macro_rules! define_first_minos {
    ( $( $var:tt =>MinoState<$type:tt, $rot:tt> ),* $(,)? ) => {
        pub const MINOS_SRC: [Minos; 7] = [
            $(
                Minos::$var(MinoState::<$type, $rot> {
                    _mino: $type,
                    _state: $rot,
                    x: 3,
                    y: 3,
                }),
            )*
        ];
    }
}

define_first_minos!(
    Is0 => MinoState<MinoI, State0>,
    Os0 => MinoState<MinoO, State0>,
    Ss0 => MinoState<MinoS, State0>,
    Zs0 => MinoState<MinoZ, State0>,
    Js0 => MinoState<MinoJ, State0>,
    Ls0 => MinoState<MinoL, State0>,
    Ts0 => MinoState<MinoT, State0>,
);
