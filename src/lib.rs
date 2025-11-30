#![cfg_attr(not(test), no_std)]

pub mod board;
mod memory;
mod mmio;
#[cfg(not(test))]
mod no_std;
pub mod uart;

#[cfg(not(test))]
pub use no_std::{init, linker, BOARD};

/// This trait exists to allow indirect access to things defined globally, usually
/// thru the [no_std::linker] mod. Doing this allows us to mock these values in tests.
trait Globals {
    fn get_uart_address(&self) -> usize;
    fn get_heap_start(&self) -> usize;
    fn get_heap_end(&self) -> usize;
}
