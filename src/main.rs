#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo, ptr};

use kernel::sbi::putchar;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

unsafe extern "C" {
    unsafe static mut __bss: u8;
    unsafe static mut __bss_end: u8;
}

#[link_section = ".text.boot"]
#[no_mangle]
#[naked_function::naked]
pub unsafe extern "C" fn boot() -> ! {
    // initialize stack pointer to correct location
    asm!(
        "la a0, __stack_top
        mv sp, a0
        j main",
    );
}

#[no_mangle]
fn main() -> ! {
    let count = &raw const __bss as usize - &raw const __bss_end as usize;
    unsafe { ptr::write_bytes(&raw mut __bss, 0, count) };

    let message = "Hello, World!";
    for c in message.chars() {
        putchar(c);
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
