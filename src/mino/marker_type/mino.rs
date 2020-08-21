use crate::*;

define_markers!(MinoI, MinoO, MinoS, MinoZ, MinoJ, MinoL, MinoT);

pub trait MinoType: NewMarker {}

impl MinoType for MinoI {}
impl MinoType for MinoO {}
impl MinoType for MinoS {}
impl MinoType for MinoZ {}
impl MinoType for MinoJ {}
impl MinoType for MinoL {}
impl MinoType for MinoT {}
