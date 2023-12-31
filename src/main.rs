#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use rust_os::{println, memory::BootInfoFrameAllocator};
use bootloader::{BootInfo, entry_point, bootinfo};


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

entry_point!(kernel_main);
 // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use rust_os::memory;
	use x86_64::{structures::paging::Page, VirtAddr};

	println!("Hello World{}", "!");
	rust_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe {memory::init(phys_mem_offset)};
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};
	
	let page = Page::containing_address(VirtAddr::new(0));
	memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};	

	#[cfg(test)]
	test_main();
	println!("NO CRASH!");
	rust_os::hlt_loop();
}
