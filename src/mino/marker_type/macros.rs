macro_rules! define_markers {
    ( $($element:tt),* $(,)? ) => {
        $(
            pub struct $element;

            impl NewMarker for $element{
                fn new() -> Self{
                    $element
                }
            }
        )*
    }
}
