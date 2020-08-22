#[macro_export]
macro_rules! define_macro_state_method {
    ( $mino:expr, $method:tt($($arg:tt)*) ) => {
        match $mino {
            Minos::Is0(m) => m.$method($($arg)*),
            Minos::Os0(m) => m.$method($($arg)*),
            Minos::Ss0(m) => m.$method($($arg)*),
            Minos::Zs0(m) => m.$method($($arg)*),
            Minos::Js0(m) => m.$method($($arg)*),
            Minos::Ls0(m) => m.$method($($arg)*),
            Minos::Ts0(m) => m.$method($($arg)*),
            Minos::IsR(m) => m.$method($($arg)*),
            Minos::OsR(m) => m.$method($($arg)*),
            Minos::SsR(m) => m.$method($($arg)*),
            Minos::ZsR(m) => m.$method($($arg)*),
            Minos::JsR(m) => m.$method($($arg)*),
            Minos::LsR(m) => m.$method($($arg)*),
            Minos::TsR(m) => m.$method($($arg)*),
            Minos::Is2(m) => m.$method($($arg)*),
            Minos::Os2(m) => m.$method($($arg)*),
            Minos::Ss2(m) => m.$method($($arg)*),
            Minos::Zs2(m) => m.$method($($arg)*),
            Minos::Js2(m) => m.$method($($arg)*),
            Minos::Ls2(m) => m.$method($($arg)*),
            Minos::Ts2(m) => m.$method($($arg)*),
            Minos::IsL(m) => m.$method($($arg)*),
            Minos::OsL(m) => m.$method($($arg)*),
            Minos::SsL(m) => m.$method($($arg)*),
            Minos::ZsL(m) => m.$method($($arg)*),
            Minos::JsL(m) => m.$method($($arg)*),
            Minos::LsL(m) => m.$method($($arg)*),
            Minos::TsL(m) => m.$method($($arg)*),
        }
    };
}
