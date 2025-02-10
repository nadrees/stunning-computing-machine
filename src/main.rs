#![no_main]
#![no_std]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use kernel::board::Board;
use kernel::println;
use kernel::uart::UART;

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
    #[cfg(feature = "board_qemu_virt")]
    let board = kernel::board::qemu_virt::VirtBoard::new();

    let uart = board.get_uart();

    println!("Hello, World!");
    loop {
        if let Some(c) = uart.read() {
            match c {
                // backspace
                8 => print!("{}{}{}", 8 as char, ' ', 8 as char),
                // new line and carriage return
                10 | 13 => println!(),
                _ => print!("{}", c as char),
            }
        }
    }
}
