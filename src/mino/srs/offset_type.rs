use crate::*;

pub trait MinoForm: PlaneNew {}

pub struct NormalTypeMino;
pub struct BarTypeMino;

impl PlaneNew for NormalTypeMino {
    fn plane() -> Self {
        NormalTypeMino
    }
}
impl PlaneNew for BarTypeMino {
    fn plane() -> Self {
        BarTypeMino
    }
}

impl MinoForm for NormalTypeMino {}
impl MinoForm for BarTypeMino {}
