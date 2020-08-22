macro_rules! define_markers {
    ( $($element:tt),* $(,)? ) => {
        $(
            #[derive(Debug, Copy, Clone)]
            pub struct $element;

            impl NewMarker for $element{
                fn new() -> Self{
                    $element
                }
            }
        )*
    }
}
