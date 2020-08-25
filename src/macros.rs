#[macro_export]
macro_rules! define_macro_state_method {
    ( $mino:expr, $method:tt($($arg:tt)*) ) => {
        match $mino {
            MinoAggregation::Is0(m) => m.$method($($arg)*),
            MinoAggregation::Os0(m) => m.$method($($arg)*),
            MinoAggregation::Ss0(m) => m.$method($($arg)*),
            MinoAggregation::Zs0(m) => m.$method($($arg)*),
            MinoAggregation::Js0(m) => m.$method($($arg)*),
            MinoAggregation::Ls0(m) => m.$method($($arg)*),
            MinoAggregation::Ts0(m) => m.$method($($arg)*),
            MinoAggregation::IsR(m) => m.$method($($arg)*),
            MinoAggregation::OsR(m) => m.$method($($arg)*),
            MinoAggregation::SsR(m) => m.$method($($arg)*),
            MinoAggregation::ZsR(m) => m.$method($($arg)*),
            MinoAggregation::JsR(m) => m.$method($($arg)*),
            MinoAggregation::LsR(m) => m.$method($($arg)*),
            MinoAggregation::TsR(m) => m.$method($($arg)*),
            MinoAggregation::Is2(m) => m.$method($($arg)*),
            MinoAggregation::Os2(m) => m.$method($($arg)*),
            MinoAggregation::Ss2(m) => m.$method($($arg)*),
            MinoAggregation::Zs2(m) => m.$method($($arg)*),
            MinoAggregation::Js2(m) => m.$method($($arg)*),
            MinoAggregation::Ls2(m) => m.$method($($arg)*),
            MinoAggregation::Ts2(m) => m.$method($($arg)*),
            MinoAggregation::IsL(m) => m.$method($($arg)*),
            MinoAggregation::OsL(m) => m.$method($($arg)*),
            MinoAggregation::SsL(m) => m.$method($($arg)*),
            MinoAggregation::ZsL(m) => m.$method($($arg)*),
            MinoAggregation::JsL(m) => m.$method($($arg)*),
            MinoAggregation::LsL(m) => m.$method($($arg)*),
            MinoAggregation::TsL(m) => m.$method($($arg)*),
        }
    };
}
