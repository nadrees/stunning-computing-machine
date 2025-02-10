pub mod ns16550;

use core::fmt::Write;

use num_traits::Unsigned;

/// An implementation of the UART (Universal Asynchronous Receiver & Transmitter)
/// interface.
pub trait UART: Write {
    /// The size of the Rx and Tx registers in the UART implementation. At most
    /// this many bits can be transmitted at once without data loss.
    type TRegisterSize: Unsigned;
}

// these are provided by the linker script
unsafe extern "C" {
    unsafe static _uart_address: usize;
}

#[inline]
fn get_uart_address() -> usize {
    &raw const _uart_address as usize
}
