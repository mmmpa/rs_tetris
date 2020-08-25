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
                    x: 3,
                    y: 3,
                }),
            )*
        ];
    }
}
