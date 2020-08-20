use crate::*;

pub trait RotationState: PlaneNew {}

pub struct State0;
pub struct StateR;
pub struct StateL;
pub struct State2;

impl PlaneNew for State0 {
    fn plane() -> Self {
        State0
    }
}
impl PlaneNew for StateR {
    fn plane() -> Self {
        StateR
    }
}
impl PlaneNew for StateL {
    fn plane() -> Self {
        StateL
    }
}
impl PlaneNew for State2 {
    fn plane() -> Self {
        State2
    }
}

impl RotationState for State0 {}
impl RotationState for StateR {}
impl RotationState for StateL {}
impl RotationState for State2 {}
