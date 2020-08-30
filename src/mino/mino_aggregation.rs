use crate::*;

#[macro_export]
macro_rules! define_mino_aggregation {
    ( $( $name:tt => $state:path ),* $(,)? ) => {
        #[derive(Debug,  Copy, Clone)]
        pub enum MinoAggregation {
            $( $name($state), )*
        }

        $(
            impl Into<MinoAggregation> for $state {
                fn into(self) -> MinoAggregation {
                    MinoAggregation::$name(self)
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! define_first_minos {
    ( $( $var:tt =>MinoState<$type:tt, $rot:tt> ),* $(,)? ) => {
        pub const MINOS_SRC: [MinoAggregation; 7] = [
            $(
                MinoAggregation::$var(MinoState::<$type, $rot> {
                    _mino: $type,
                    _state: $rot,
                    x: MINO_FIRST_POSITION.0,
                    y: MINO_FIRST_POSITION.1,
                }),
            )*
        ];

        pub const MINOS_SRC_ZERO_POSITION: [MinoAggregation; 7] = [
            $(
                MinoAggregation::$var(MinoState::<$type, $rot> {
                    _mino: $type,
                    _state: $rot,
                    x: 1,
                    y: 1,
                }),
            )*
        ];
  }
}
