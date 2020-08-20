use crate::*;

pub struct SrsOffset<T: MinoForm, A: RotationState, B: RotationState> {
    t: T,
    a: A,
    b: B,
}

pub trait SrsOffsetExe {
    type Form: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    fn offset() -> &'static [(i8, i8)];
}

#[macro_export]
macro_rules! define_srs_offset {
    ( $form:tt, $from:tt => $to:tt, $candidates:tt ) => {
        impl SrsOffsetExe for SrsOffset<$form, $from, $to> {
            type Form = $form;
            type Now = $from;
            type Next = $to;

            fn offset() -> &'static [(i8, i8)] {
                &$candidates
            }
        }
    };
}

// Bar

define_srs_offset!(BarTypeMino, State0 => StateR, [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]);
define_srs_offset!(BarTypeMino, StateR => State0, [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]);

define_srs_offset!(BarTypeMino, StateR => State2, [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]);
define_srs_offset!(BarTypeMino, State2 => StateR, [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]);

define_srs_offset!(BarTypeMino, State2 => StateL, [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]);
define_srs_offset!(BarTypeMino, StateL => State2, [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]);

define_srs_offset!(BarTypeMino, StateL => State0, [(0, 0), (1, 0), (-2, -1), (1, -2), (-2, 1)]);
define_srs_offset!(BarTypeMino, State0 => StateL, [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]);

// Other than bar

define_srs_offset!(NormalTypeMino, State0 => StateR, [(0, 0), (-1, 0), (-1, -1), (0, -2), (-1, -2)]);
define_srs_offset!(NormalTypeMino, StateR => State0, [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]);

define_srs_offset!(NormalTypeMino, StateR => State2, [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]);
define_srs_offset!(NormalTypeMino, State2 => StateR, [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]);

define_srs_offset!(NormalTypeMino, State2 => StateL, [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]);
define_srs_offset!(NormalTypeMino, StateL => State2, [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]);

define_srs_offset!(NormalTypeMino, StateL => State0, [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]);
define_srs_offset!(NormalTypeMino, State0 => StateL, [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]);
