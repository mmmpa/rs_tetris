use crate::*;

#[rustfmt::skip]
pub trait Rotatable { fn is_rotatable(&self) -> bool { true } }

impl<Rot: RotationState> Rotatable for MinoState<MinoI, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoS, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoZ, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoJ, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoL, Rot> {}
impl<Rot: RotationState> Rotatable for MinoState<MinoT, Rot> {}

#[rustfmt::skip]
impl<Rot: RotationState> Rotatable for MinoState<MinoO, Rot> { fn is_rotatable(&self) -> bool { false } }
