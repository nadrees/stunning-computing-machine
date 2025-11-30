#![no_std]

pub mod board;
mod linker;
mod memory;
mod mmio;
pub mod uart;

/// Performs all initialization that needs to be
/// done prior to the application starting.
pub fn init() {
    memory::init();
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use kernel::board::BOARD;
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
