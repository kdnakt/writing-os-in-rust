// disable standard library, not to use libc etc.
#![no_std]
// overwriting the entry point
#![no_main]

use core::panic::PanicInfo;

// fn main() {
//     // println!("Hello, world!");
// }

// ! is the "never" type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World";

// disable name mangling to ensure the name "_start"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
