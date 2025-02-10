#![no_main]
#![no_std]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

global_asm!(include_str!("boot.S"));

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}

#[no_mangle]
pub fn rs_main() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}
