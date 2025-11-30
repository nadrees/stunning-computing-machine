#![no_main]
#![no_std]

extern crate alloc;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use kernel::println;
use kernel::uart::UART;
use kernel::BOARD;
use kernel::{board::Board, init};

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
    init();

    println!("Hello, World!");
    loop {
        let read_result = {
            let lock = BOARD.lock();
            lock.get_uart().read()
        };
        if let Some(c) = read_result {
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
