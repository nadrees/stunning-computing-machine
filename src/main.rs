#![no_main]
#![no_std]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use kernel::println;

global_asm!(include_str!("boot.S"));

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    if let Some(location) = panic.location() {
        println!(
            "{} ({}): {}",
            location.file(),
            location.line(),
            panic.message()
        );
    }
    loop {
        unsafe { asm!("wfi") }
    }
}

#[no_mangle]
pub fn rs_main() -> ! {
    println!("Hello, World!");
    loop {
        unsafe { asm!("wfi") }
    }
}
