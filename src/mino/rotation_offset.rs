use crate::*;

pub struct RotationOffset<T: MinoForm, A: RotationState, B: RotationState>(T, A, B);

/// Return offsets after rotation for minos from state to state.
///
/// Associated types are just for binding types in other trait definition.
pub trait RotationOffsetExe {
    type Form: MinoForm;
    type Now: RotationState;
    type Next: RotationState;

    fn offset() -> (i8, i8);
}

#[macro_export]
macro_rules! define_rotation_offset {
    ( $form:tt, $from:tt => $to:tt, $pos:tt ) => {
        impl RotationOffsetExe for RotationOffset<$form, $from, $to> {
            type Form = $form;
            type Now = $from;
            type Next = $to;

            fn offset() -> (i8, i8) {
                $pos
            }
        }
    };
}

define_rotation_offset!(BarTypeMino, State0 => StateR, (2, -1));
define_rotation_offset!(BarTypeMino, StateR => State0, (-2, 1));
define_rotation_offset!(BarTypeMino, StateR => State2, (-2, 2));
define_rotation_offset!(BarTypeMino, State2 => StateR, (2, -2));
define_rotation_offset!(BarTypeMino, State2 => StateL, (1, -2));
define_rotation_offset!(BarTypeMino, StateL => State2, (-1, 2));
define_rotation_offset!(BarTypeMino, StateL => State0, (-1, 1));
define_rotation_offset!(BarTypeMino, State0 => StateL, (1, -1));

define_rotation_offset!(NormalTypeMino, State0 => StateR, (1, 0));
define_rotation_offset!(NormalTypeMino, StateR => State0, (-1, 0));
define_rotation_offset!(NormalTypeMino, StateR => State2, (-1, 1));
define_rotation_offset!(NormalTypeMino, State2 => StateR, (1, -1));
define_rotation_offset!(NormalTypeMino, State2 => StateL, (0, -1));
define_rotation_offset!(NormalTypeMino, StateL => State2, (0, 1));
define_rotation_offset!(NormalTypeMino, StateL => State0, (0, 0));
define_rotation_offset!(NormalTypeMino, State0 => StateL, (0, 0));

define_rotation_offset!(OTypeMino, State0 => StateR, (0,0));
define_rotation_offset!(OTypeMino, StateR => State0, (0,0));
define_rotation_offset!(OTypeMino, StateR => State2, (0,0));
define_rotation_offset!(OTypeMino, State2 => StateR, (0,0));
define_rotation_offset!(OTypeMino, State2 => StateL, (0,0));
define_rotation_offset!(OTypeMino, StateL => State2, (0,0));
define_rotation_offset!(OTypeMino, StateL => State0, (0,0));
define_rotation_offset!(OTypeMino, State0 => StateL, (0,0));
