#[macro_export]
macro_rules! log {
    ($format:literal $(, $val:expr)*) => {
        println!("\x1b[48;5;3;38;5;16m woka \x1b[;38;5;3m {}\x1b[m", format!($format, $($val, )*))
    };
}

