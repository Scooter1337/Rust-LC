// L.A. (Luca) Verheul - S3704041
// Wed 13 Dec 2023

/// A macro to print debug information only in debug mode.
/// When running the program in release mode, the macro will be replaced with nothing.
/// The macro is evaluated at compile time, so there is 0.0 runtime cost.
#[macro_export]
macro_rules! dbg {
    ($($expr:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($expr),+)
            }
        }
    };
}
