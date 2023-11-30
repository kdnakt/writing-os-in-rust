// disable standard library, not to use libc etc.
#![no_std]

use core::panic::PanicInfo;

fn main() {
    // println!("Hello, world!");
}

// ! is the "never" type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}