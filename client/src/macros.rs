// Requires 'colored' create
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "ERROR:".red(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "INFO:".blue(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "DEBUG:".yellow(), format!($($arg)*))
    };
}
