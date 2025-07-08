#![no_std]
#![no_main]

mod util;
mod vga_writer;

use core::panic::PanicInfo;
use vga_writer::VGAWriter;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let textetc = b"
        abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789
        abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut writer = VGAWriter::new();

    writer.write_bytes(textetc);
    writer.write_str("alskeeeeee2ëdjfslkdfj");

    loop {}
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}
