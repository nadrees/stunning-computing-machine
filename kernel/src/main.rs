#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn boot() -> ! {
    loop {}
}
