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

    blog_os::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // panic!("Some panic message");

    // trigger a double fault
    // unsafe {
    //     // deadbeef is an invalid address, so a page fault occurs.
    //     // if our IDT doesn't have a page fault handler, a double fault occurs.
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // trigger stack overflow
    // fn stack_overflow() {
    //     // for each recursion, the return address is pushed
    //     stack_overflow();
    // }
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
