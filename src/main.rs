// disable standard library, not to use libc etc.
#![no_std]
// overwriting the entry point
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

// fn main() {
//     // println!("Hello, world!");
// }

// ! is the "never" type
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

// static HELLO: &[u8] = b"Hello World";

// disable name mangling to ensure the name "_start"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    println!("Hello World{}", "!");

    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
