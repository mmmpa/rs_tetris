use core::fmt::Debug;

pub trait NewMarker: Debug {
    fn new() -> Self;
}
