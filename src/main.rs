// disable standard library, not to use libc etc.
#![no_std]
// overwriting the entry point
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{
    boxed::Box,
    vec,
    vec::Vec,
    rc::Rc,
};
use bootloader::{
    BootInfo,
    entry_point,
};
use core::{
    panic::PanicInfo,
    pin::Pin,
    marker::PhantomPinned,
};
use blog_os::println;
use blog_os::task::{
    Task,
    keyboard,
    simple_executor::SimpleExecutor,
    executor::Executor,
};

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
    use blog_os::allocator;
    use blog_os::memory;
    use blog_os::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::{
        Translate,
        Page,
    };

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

    // use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let page = Page::containing_address(VirtAddr::new(0));
    // frame allocation fails for wrong address
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // print `New!`
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    let addresses = [
        // vga buffer
        0xb8000,
        // code page
        0x201008,
        // stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address "0"
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // allocation error test
    // let x = Box::new(41);

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    let mut heap_value = Box::pin(SelfReferential {
        self_ptr: 0 as *const _,
        _pin: PhantomPinned,
    });
    let ptr = &*heap_value as *const SelfReferential;
    // heap_value.self_ptr = ptr;
    unsafe {
        let mut_ref = Pin::as_mut(&mut heap_value);
        Pin::get_unchecked_mut(mut_ref).self_ptr = ptr;
    }
    println!("heap value at: {:p}", heap_value);
    println!("internal reference: {:p}", heap_value.self_ptr);
    // let stack_value = core::mem::replace(&mut *heap_value, SelfReferential {
    //     self_ptr: 0 as *const _,
    // });
    // println!("value at: {:p}", &stack_value);
    // println!("internal reference: {:p}", stack_value.self_ptr);

    // let mut executor = SimpleExecutor::new();
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    // loop {
    //     use blog_os::print;
    //     print!("-"); // this may cause deadlock
    // }
    blog_os::hlt_loop();
}

struct SelfReferential {
    self_ptr: * const Self,
    _pin: PhantomPinned,
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let num = async_number().await;
    println!("async number: {}", num);
}
