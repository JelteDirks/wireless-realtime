#![no_std]
#![no_main]

mod util;
mod vga_writer;

use core::panic::PanicInfo;

use crate::util::delay;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    println!("deeznutsalskdjflksdjf {}", 10);

    let mut c = 0;
    loop {
        c += 1;
        for i in 0..100 {
            delay();
        }
        println!("looping {}", c);
    }
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}
