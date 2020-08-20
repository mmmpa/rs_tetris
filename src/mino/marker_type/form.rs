use crate::*;

define_markers!(NormalTypeMino, BarTypeMino);

pub trait MinoForm: PlaneNew {}

impl MinoForm for NormalTypeMino {}
impl MinoForm for BarTypeMino {}
