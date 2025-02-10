#![no_std]

pub mod board;
mod mmio;
mod uart;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {
        use kernel::board::Board;
        use core::fmt::Write;

        let mut board = {
            #[cfg(feature = "board_qemu_virt")]
            {
                use kernel::board::qemu_virt::VirtBoard;
                VirtBoard::new()
            }
        };
        let _ = write!(board.get_uart_mut(), $($args)+);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        use kernel::print;
        print!("\r\n")
    };
    ($fmt:expr) => {
        use kernel::print;
        print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($args:tt)+) => {
        use kernel::print;
        print!(concat!($fmt, "\r\n"), $($args)+)
    };
}
