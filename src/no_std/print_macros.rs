//! These macros are defined here so that they dont get included during tests, which would cause
//! duplicate definitiosn for print! and println!

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use crate::BOARD;
        use core::fmt::Write;

        let _ = write!(BOARD.lock().get_uart_mut(), $($args)+);
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        use kernel::print;
        print!("\r\n")
    });
    ($fmt:expr) => {
        use kernel::print;
        print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($args:tt)+) => {
        use kernel::print;
        print!(concat!($fmt, "\r\n"), $($args)+)
    };
}
