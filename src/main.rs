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

// disable name mangling to ensure the name "_start"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
