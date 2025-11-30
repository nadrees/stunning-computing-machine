//! This module contains any code that is only valid when compiling in a no_std environment. Usually this
//! is becuase it depends on global variables defined in linker scripts, has RISC-V specific details, or
//! otherwise wouldn't work in the std env used in tests.
//! Notably, this means that anything here *cannot* be used during tests.

use spin::{Lazy, Mutex};

use crate::{board::qemu_virt::VirtBoard, linker::LinkerGlobals, memory::Allocator};

pub mod linker;
mod print_macros;

static LINKER_GLOBALS: LinkerGlobals = LinkerGlobals;

#[global_allocator]
static ALLOCATOR: Allocator<LinkerGlobals> = Allocator {
    globals: &LINKER_GLOBALS,
};

pub static BOARD: Lazy<Mutex<VirtBoard>> =
    Lazy::new(|| Mutex::new(VirtBoard::new(&LINKER_GLOBALS)));

/// Performs all initialization that needs to be
/// done prior to the application starting.
pub fn init() {
    ALLOCATOR.init();
}
