use crate::{
    linker::get_uart_address,
    uart::{ns16550::NS16550, UART},
};

use super::Board;

/// The board for the QEMU virtual board
pub struct VirtBoard {
    // the QEMU virt board emulates the NS16550 chip
    // for UART: https://www.qemu.org/docs/master/system/riscv/virt.html
    uart: NS16550,
}

impl VirtBoard {
    /// Constructs a new board, and initializes the peripherals.
    pub fn new() -> Self {
        Self {
            uart: NS16550::new(get_uart_address()),
        }
    }
}

impl Board<u8> for VirtBoard {
    fn get_uart_mut(&mut self) -> &mut impl UART<TRegisterSize = u8> {
        &mut self.uart
    }

    fn get_uart(&self) -> &impl UART<TRegisterSize = u8> {
        &self.uart
    }
}
