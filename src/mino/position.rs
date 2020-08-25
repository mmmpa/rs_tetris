use crate::*;

pub trait Position {
    /// for cloning
    fn pos(&self) -> (i8, i8);

    /// for moving and rotation
    fn absolute(&mut self, xy: (i8, i8));
    fn offset(&mut self, xy: (i8, i8));
}

impl<MT: MinoType, Rot: RotationState> Position for MinoState<MT, Rot> {
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
