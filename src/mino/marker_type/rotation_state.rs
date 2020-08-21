use crate::*;

define_markers!(State0, StateR, StateL, State2);

pub trait RotationState: NewMarker {}

impl RotationState for State0 {}
impl RotationState for StateR {}
impl RotationState for StateL {}
impl RotationState for State2 {}
