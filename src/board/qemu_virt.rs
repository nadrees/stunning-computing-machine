use crate::uart::{ns16550::NS16550, UART};

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
            uart: NS16550::new(),
        }
    }
}

impl Board<u8> for VirtBoard {
    fn get_uart_mut(&mut self) -> &mut impl UART<TRegisterSize = u8> {
        &mut self.uart
    }
}
