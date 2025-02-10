pub mod ns16550;

use core::fmt::Write;

use num_traits::Unsigned;

/// An implementation of the UART (Universal Asynchronous Receiver & Transmitter)
/// interface.
pub trait UART: Write {
    /// The size of the Rx and Tx registers in the UART implementation. At most
    /// this many bits can be transmitted at once without data loss.
    type TRegisterSize: Unsigned;

    /// Attempt to read from the UART. If data is available, it will be returned,
    /// otherwise None will be returned.
    fn read(&self) -> Option<Self::TRegisterSize>;
}
