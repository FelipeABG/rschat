// Requires 'colored' create
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", ::colored::Colorize::red("ERROR:"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{} {}", ::colored::Colorize::blue("INFO:"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        eprintln!("{} {}", ::colored::Colorize::yellow("DEBUG:"), format!($($arg)*))
    };
}
