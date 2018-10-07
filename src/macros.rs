/// Simple macro to format a hidden bool
#[macro_export]
macro_rules! fmt_hidden {
    ($hidden:expr) => {
        if $hidden {
            "including hidden"
        } else {
            "excluding hidden"
        }
    };
}
