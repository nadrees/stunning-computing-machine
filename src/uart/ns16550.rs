use core::fmt::Write;

use super::UART;
use crate::mmio::read as mmio_read;
use crate::mmio::write as mmio_write;

/// THR = Transmitted Holding Register (write-only), RHR = Reciever Holding Register (read-only).
/// THR is what we write to when sending data, RHR is what we read from
/// to read data.
const THR_RHR_ADDRESS: usize = 0;
/// IER = Interrupt Enable Register
const IER_ADDRESS: usize = 1;
/// FIFO = FIFO Control Register (write-only), ISR = Interrupt Status Register (read-only)
const FIFO_ISR_ADDRESS: usize = 2;
/// LCR = Line Control Register
const LCR_ADDRESS: usize = 3;

/// Masks to check when reading the ISR to determine what kind of interrupt (if any)
/// we're handling. These should be applied to the lowest 4 bits
mod isr_masks {
    /// Mask for if there's data ready to be read
    pub const RECEIVED_DATA_READY_MASK: u8 = 0b0100;
}

pub struct NS16550 {
    address: usize,
}

impl UART for NS16550 {
    // this chip has 8 bit registers: https://caro.su/msx/ocm_de1/16550.pdf
    type TRegisterSize = u8;

    fn read(&self) -> Option<Self::TRegisterSize> {
        let result = mmio_read::<u8>(self.address + FIFO_ISR_ADDRESS, 0);
        if result & isr_masks::RECEIVED_DATA_READY_MASK != isr_masks::RECEIVED_DATA_READY_MASK {
            return None;
        }
        Some(mmio_read::<u8>(self.address + THR_RHR_ADDRESS, 0))
    }
}

impl Write for NS16550 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.write_register(c);
        }
        Ok(())
    }
}

impl NS16550 {
    pub fn new(address: usize) -> Self {
        // initialize LCR (line control register)
        // we need bits 0 & 1 to be 1 for 8-bit mode
        // and we need the rest of the bits to 0
        mmio_write::<u8>(address + LCR_ADDRESS, 0, 0b00000011);

        // enable fifo (bit 0), and reset both receiver (bit 1) and transmitter (bit 2)
        mmio_write::<u8>(address + FIFO_ISR_ADDRESS, 0, 0b111);

        // enable IER data ready interrupt (bit 0)
        mmio_write::<u8>(address + IER_ADDRESS, 0, 0b1);

        Self { address }
    }

    fn write_register(&mut self, value: <NS16550 as UART>::TRegisterSize) {
        mmio_write(self.address + THR_RHR_ADDRESS, 0, value);
    }
}
