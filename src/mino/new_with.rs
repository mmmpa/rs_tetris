use crate::*;

pub trait NewWith {
    fn new_with(xy: (i8, i8)) -> Self;
}

impl<MT: MinoType, Rot: RotationState> NewWith for MinoState<MT, Rot> {
    /// for cloning after rotation
    fn new_with((x, y): (i8, i8)) -> Self {
        MinoState::<MT, Rot>::new(x, y)
    }
}
