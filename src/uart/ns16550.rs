use core::fmt::Write;

use super::{get_uart_address, UART};
use crate::mmio::write as mmio_write;

/// THR = Transmitted Holding Register, RHR = Reciever Holding Register.
/// THR is what we write to when sending data, RHR is what we read from
/// to read data.
const THR_RHR_ADDRESS: usize = 0;
/// IER = Interrupt Enable Register
const IER_ADDRESS: usize = 1;
/// FIFO = FIFO Control Register
const FIFO_ADDRESS: usize = 2;
/// LCR = Line Control Register
const LCR_ADDRESS: usize = 3;

pub struct NS16550 {}

impl UART for NS16550 {
    // this chip has 8 bit registers: https://caro.su/msx/ocm_de1/16550.pdf
    type TRegisterSize = u8;
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
    pub fn new() -> Self {
        let address = get_uart_address();

        // initialize LCR (line control register)
        // we need bits 0 & 1 to be 1 for 8-bit mode
        // and we need the rest of the bits to 0
        mmio_write::<u8>(address + LCR_ADDRESS, 0, 0b00000011);

        // enable fifo (bit 0), and reset both receiver (bit 1) and transmitter (bit 2)
        mmio_write::<u8>(address + FIFO_ADDRESS, 0, 0b111);

        // enable IER data ready interrupt (bit 0)
        mmio_write::<u8>(address + IER_ADDRESS, 0, 0b1);

        Self {}
    }

    fn write_register(&mut self, value: <NS16550 as UART>::TRegisterSize) {
        let address = get_uart_address();
        mmio_write(address + THR_RHR_ADDRESS, 0, value);
    }
}
