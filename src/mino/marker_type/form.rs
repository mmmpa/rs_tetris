use crate::*;

define_markers!(NormalTypeMino, BarTypeMino);

pub trait MinoForm: NewMarker {}

impl MinoForm for NormalTypeMino {}
impl MinoForm for BarTypeMino {}
