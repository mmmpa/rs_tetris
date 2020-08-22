use crate::*;
use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct MinoState<MT: MinoType, MF: MinoForm, Rot: RotationState> {
    _mino: MT,
    _form: MF,
    _state: Rot,
    x: i8,
    y: i8,
}

pub trait NewWithPos {
    fn new_with(x: i8, y: i8) -> Self;
    fn new_with_t(xy: (i8, i8)) -> Self;
}

impl<MT: MinoType, MF: MinoForm, Rot: RotationState> NewWithPos for MinoState<MT, MF, Rot> {
    fn new_with(x: i8, y: i8) -> Self {
        Self {
            _mino: MT::new(),
            _form: MF::new(),
            _state: Rot::new(),
            x,
            y,
        }
    }

    /// for cloning after rotation
    fn new_with_t((x, y): (i8, i8)) -> Self {
        Self::new_with(x, y)
    }
}

pub trait MinoFn: MinoCore + Right + Left {}

/// Provide a mino information for rendering.
pub trait MinoCore: NewWithPos + Into<Minos> + Debug {
    type Mino: MinoType;
    type Form: MinoForm;
    type Now: RotationState;
    type Right: RotationState;
    type Side: RotationState;
    type Left: RotationState;
    type Cell: CellExe<Mino = Self::Mino, State = Self::Now>;

    /// for cloning
    fn pos(&self) -> (i8, i8);

    /// for moving
    fn absolute(&mut self, xy: (i8, i8));
    fn offset(&mut self, xy: (i8, i8));

    /// for rendering
    fn mut_with_absolute_cells<F>(&self, f: F)
    where
        F: FnMut(i8, i8);

    /// For hit testing.
    /// Returning false means that all cells don't hit.
    fn test_with_absolute_cells<F>(&self, f: F) -> bool
    where
        F: Fn(i8, i8) -> bool;

    fn is_0(&self) -> bool;
    fn is_r(&self) -> bool;
    fn is_l(&self) -> bool;
    fn is_2(&self) -> bool;
}

macro_rules! define_mino_common {
    () => {
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

        fn mut_with_absolute_cells<F>(&self, mut f: F)
        where
            F: FnMut(i8, i8),
        {
            Self::Cell::cells()
                .iter()
                .for_each(|(x, y)| f(self.x + x, self.y + y));
        }

        fn test_with_absolute_cells<F>(&self, f: F) -> bool
        where
            F: Fn(i8, i8) -> bool,
        {
            for (x, y) in Self::Cell::cells().iter() {
                if f(self.x + x, self.y + y) {
                    return true;
                }
            }

            false
        }
    };
}

macro_rules! define_mino_rotation {
    ( $mino_type:tt, $mino_form:tt, $direction:tt, $from:tt => $to:tt ) => {
        impl $direction for MinoState<$mino_type, $mino_form, $from> {
            type Next = MinoState<$mino_type, $mino_form, $to>;
            type Rotation = RotationOffset<$mino_form, $from, $to>;
            type Srs = SrsOffset<$mino_form, $from, $to>;
        }
    };
}

macro_rules! define_mino {
    ( $mino_type:tt, $mino_form:tt ) => {
        impl MinoCore for MinoState<$mino_type, $mino_form, State0> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = State0;
            type Right = StateR;
            type Side = State2;
            type Left = StateL;
            type Cell = Cell<$mino_type, State0>;

            fn is_0(&self) -> bool { true }
            fn is_r(&self) -> bool { false }
            fn is_l(&self) -> bool { false }
            fn is_2(&self) -> bool { false }

            define_mino_common!();
        }

        impl MinoCore for MinoState<$mino_type, $mino_form, StateR> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = StateR;
            type Right = State2;
            type Side = StateL;
            type Left = State0;
            type Cell = Cell<$mino_type, StateR>;

            fn is_0(&self) -> bool { false }
            fn is_r(&self) -> bool { true }
            fn is_l(&self) -> bool { false }
            fn is_2(&self) -> bool { false }

           define_mino_common!();
        }

        impl MinoCore for MinoState<$mino_type, $mino_form, StateL> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = StateL;
            type Right = State0;
            type Side = StateR;
            type Left = State2;
            type Cell = Cell<$mino_type, StateL>;

            fn is_0(&self) -> bool { false }
            fn is_r(&self) -> bool { false }
            fn is_l(&self) -> bool { true }
            fn is_2(&self) -> bool { false }

          define_mino_common!();
        }

        impl MinoCore for MinoState<$mino_type, $mino_form, State2> {
            type Mino = $mino_type;
            type Form = $mino_form;
            type Now = State2;
            type Right = StateL;
            type Side = State0;
            type Left = StateR;
            type Cell = Cell<$mino_type, State2>;

            fn is_0(&self) -> bool { false }
            fn is_r(&self) -> bool { false }
            fn is_l(&self) -> bool { false }
            fn is_2(&self) -> bool { true }

            define_mino_common!();
        }

        define_mino_rotation!($mino_type, $mino_form, Right, State0 => StateR);
        define_mino_rotation!($mino_type, $mino_form, Right, StateR => State2);
        define_mino_rotation!($mino_type, $mino_form, Right, State2 => StateL);
        define_mino_rotation!($mino_type, $mino_form, Right, StateL => State0);

        define_mino_rotation!($mino_type, $mino_form, Left, State0 => StateL);
        define_mino_rotation!($mino_type, $mino_form, Left, StateL => State2);
        define_mino_rotation!($mino_type, $mino_form, Left, State2 => StateR);
        define_mino_rotation!($mino_type, $mino_form, Left, StateR => State0);

        impl MinoFn for MinoState<$mino_type, $mino_form, State0> {}
        impl MinoFn for MinoState<$mino_type, $mino_form, StateR> {}
        impl MinoFn for MinoState<$mino_type, $mino_form, State2> {}
        impl MinoFn for MinoState<$mino_type, $mino_form, StateL> {}
  };
}

define_mino!(MinoI, BarTypeMino);
define_mino!(MinoO, NormalTypeMino);
define_mino!(MinoS, NormalTypeMino);
define_mino!(MinoZ, NormalTypeMino);
define_mino!(MinoJ, NormalTypeMino);
define_mino!(MinoL, NormalTypeMino);
define_mino!(MinoT, NormalTypeMino);

macro_rules! define_minos {
    ( $( $name:tt => $state:path ),* $(,)? ) => {
        #[derive(Debug,  Copy, Clone)]
        pub enum Minos {
            $( $name($state), )*
        }
        $(
            impl Into<Minos> for $state {
                fn into(self) -> Minos {
                    Minos::$name(self)
                }
            }
        )*
    }
}

define_minos!(
    Is0 => MinoState<MinoI, BarTypeMino, State0>,
    Os0 => MinoState<MinoO, NormalTypeMino, State0>,
    Ss0 => MinoState<MinoS, NormalTypeMino, State0>,
    Zs0 => MinoState<MinoZ, NormalTypeMino, State0>,
    Js0 => MinoState<MinoJ, NormalTypeMino, State0>,
    Ls0 => MinoState<MinoL, NormalTypeMino, State0>,
    Ts0 => MinoState<MinoT, NormalTypeMino, State0>,
    IsR => MinoState<MinoI, BarTypeMino, StateR>,
    OsR => MinoState<MinoO, NormalTypeMino, StateR>,
    SsR => MinoState<MinoS, NormalTypeMino, StateR>,
    ZsR => MinoState<MinoZ, NormalTypeMino, StateR>,
    JsR => MinoState<MinoJ, NormalTypeMino, StateR>,
    LsR => MinoState<MinoL, NormalTypeMino, StateR>,
    TsR => MinoState<MinoT, NormalTypeMino, StateR>,
    Is2 => MinoState<MinoI, BarTypeMino, State2>,
    Os2 => MinoState<MinoO, NormalTypeMino, State2>,
    Ss2 => MinoState<MinoS, NormalTypeMino, State2>,
    Zs2 => MinoState<MinoZ, NormalTypeMino, State2>,
    Js2 => MinoState<MinoJ, NormalTypeMino, State2>,
    Ls2 => MinoState<MinoL, NormalTypeMino, State2>,
    Ts2 => MinoState<MinoT, NormalTypeMino, State2>,
    IsL => MinoState<MinoI, BarTypeMino, StateL>,
    OsL => MinoState<MinoO, NormalTypeMino, StateL>,
    SsL => MinoState<MinoS, NormalTypeMino, StateL>,
    ZsL => MinoState<MinoZ, NormalTypeMino, StateL>,
    JsL => MinoState<MinoJ, NormalTypeMino, StateL>,
    LsL => MinoState<MinoL, NormalTypeMino, StateL>,
    TsL => MinoState<MinoT, NormalTypeMino, StateL>,
);

macro_rules! define_first_minos {
    ( $( $var:tt =>MinoState<$type:tt, $form:tt, $rot:tt> ),* $(,)? ) => {
        pub const MINOS_SRC: [Minos; 7] = [
            $(
                Minos::$var(MinoState::<$type, $form, $rot> {
                    _mino: $type,
                    _form: $form,
                    _state: $rot,
                    x: 3,
                    y: 3,
                }),
            )*
        ];
    }
}

define_first_minos!(
    Is0 => MinoState<MinoI, BarTypeMino, State0>,
    Os0 => MinoState<MinoO, NormalTypeMino, State0>,
    Ss0 => MinoState<MinoS, NormalTypeMino, State0>,
    Zs0 => MinoState<MinoZ, NormalTypeMino, State0>,
    Js0 => MinoState<MinoJ, NormalTypeMino, State0>,
    Ls0 => MinoState<MinoL, NormalTypeMino, State0>,
    Ts0 => MinoState<MinoT, NormalTypeMino, State0>,
);
