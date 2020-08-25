use crate::*;

pub trait IsState {
    fn is_0(&self) -> bool;
    fn is_r(&self) -> bool;
    fn is_l(&self) -> bool;
    fn is_2(&self) -> bool;
}

#[rustfmt::skip]
impl<MT: MinoType> IsState for MinoState<MT, State0> {
    fn is_0(&self) -> bool { true }
    fn is_r(&self) -> bool { false }
    fn is_l(&self) -> bool { false }
    fn is_2(&self) -> bool { false }
}

#[rustfmt::skip]
impl<MT: MinoType> IsState for MinoState<MT, StateR> {
    fn is_0(&self) -> bool { false }
    fn is_r(&self) -> bool { true }
    fn is_l(&self) -> bool { false }
    fn is_2(&self) -> bool { false }
}

#[rustfmt::skip]
impl<MT: MinoType> IsState for MinoState<MT, StateL> {
    fn is_0(&self) -> bool { false }
    fn is_r(&self) -> bool { false }
    fn is_l(&self) -> bool { true }
    fn is_2(&self) -> bool { false }
}

#[rustfmt::skip]
impl<MT: MinoType> IsState for MinoState<MT, State2> {
    fn is_0(&self) -> bool { false }
    fn is_r(&self) -> bool { false }
    fn is_l(&self) -> bool { false }
    fn is_2(&self) -> bool { true }
}
