use crate::uart::UART;

pub mod qemu_virt;

/// The base trait all boards are expected to implement. Provides handles
/// to the various devices common to all boards.
pub trait Board<UARTSize> {
    /// Gets a mutable reference to the [UART] implementation for this board.
    /// This allows us to call the write methods on the UART interface to send
    /// data.
    fn get_uart_mut(&mut self) -> &mut impl UART<TRegisterSize = UARTSize>;
}
