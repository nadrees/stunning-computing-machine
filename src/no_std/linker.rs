//! This module wraps any values provided by the kernel.ld script directly.
//! Changes to this file must be synchronized with changes to that, and vice versa.

// these are provided by the linker script

use crate::Globals;
unsafe extern "C" {
    unsafe static _uart_address: usize;
    unsafe static _heap_start: usize;
    unsafe static _heap_end: usize;
}

macro_rules! make_get_linker_variable {
    ($name:ident) => {
        pastey::paste! {
            #[inline]
            fn [<get $name>](&self) -> usize {
                &raw const $name as usize
            }
        }
    };
}

/// The version of the globals provided from the linker script.
pub struct LinkerGlobals;

impl Globals for LinkerGlobals {
    make_get_linker_variable!(_uart_address);
    make_get_linker_variable!(_heap_start);
    make_get_linker_variable!(_heap_end);
}
