#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo, ptr};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn boot() -> ! {
    unsafe extern "C" {
        unsafe static mut __bss: u8;
        unsafe static mut __bss_end: u8;
        unsafe static __stack_top: u8;
    }

    // initialize stack pointer to correct location
    unsafe {
        asm!(
            "mv sp, {sp}",
            sp = in(reg) &__stack_top
        );
    };

    let count = &raw const __bss as usize - &raw const __bss_end as usize;
    ptr::write_bytes(&raw mut __bss, 0, count);

    main();
}

#[no_mangle]
fn main() -> ! {
    loop {}
}
