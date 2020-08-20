macro_rules! define_markers {
    ( $($element:tt),* $(,)? ) => {
        $(

        pub struct $element;
        impl PlaneNew for $element{
            fn plane() -> Self{
                $element
            }
        }

        )*
        }
}
