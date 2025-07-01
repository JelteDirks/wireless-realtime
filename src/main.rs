#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER_ADDR:u32 = 0xb8000;
const LIGHT_CYAN: u8 =  0xb;

static HELLOWORLD: &[u8] = b"DEEZ NUTS";


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // casts to pointer of u8 to write into the buffer one u8 at a time
    let vga_buffer_start = VGA_BUFFER_ADDR as *mut u8;

    for (i, byte) in HELLOWORLD.iter().enumerate() {
        unsafe {
            *vga_buffer_start.offset(i as isize * 2) = *byte;
            *vga_buffer_start.offset(i as isize * 2 + 1) = LIGHT_CYAN;
        }
    }

    loop {}
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}
