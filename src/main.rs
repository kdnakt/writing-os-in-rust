// disable standard library, not to use libc etc.
#![no_std]
// overwriting the entry point
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{
    BootInfo,
    entry_point,
};
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
    // loop {}
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

entry_point!(kernel_main);

// static HELLO: &[u8] = b"Hello World";

// disable name mangling to ensure the name "_start"
#[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
fn kernel_main(boot_info: &'static BootInfo) -> ! {
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

    // cause page fault by accessing memory outside the kernel
    // let ptr = 0xdeadbeaf as *mut u8;
    // let ptr = 0x20514b as *mut u8;
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // unsafe { *ptr = 42; }
    // println!("write worked");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    // loop {
    //     use blog_os::print;
    //     print!("-"); // this may cause deadlock
    // }
    blog_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
