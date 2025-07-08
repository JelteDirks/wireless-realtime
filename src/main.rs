#![no_std]
#![no_main]

mod util;
mod vga_writer;

use core::panic::PanicInfo;

use crate::util::delay;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("deeznutsalskdjflksdjf {}", 10);
    panic!();
}

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
