use crate::*;

define_markers!(NormalTypeMino, BarTypeMino, OTypeMino);

pub trait MinoForm: NewMarker {}

impl MinoForm for NormalTypeMino {}
impl MinoForm for BarTypeMino {}
impl MinoForm for OTypeMino {}
