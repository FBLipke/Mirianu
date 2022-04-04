

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use mirianu_os::println;
use bootloader::{BootInfo, entry_point};
extern crate alloc;
use mirianu_os::task::Task;
use mirianu_os::task::executor::Executor;
use mirianu_os::task::keyboard;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use mirianu_os::memory;
	use x86_64::{structures::paging::Page, VirtAddr};
	use mirianu_os::memory::BootInfoFrameAllocator;	
	use mirianu_os::allocator;

	println!("Mirianu 0.1");
	println!("[I] Booting...");
	
	mirianu_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe {
        	BootInfoFrameAllocator::init(&boot_info.memory_map)
    	};

	let page = Page::containing_address(VirtAddr::new(0));
    	memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
	allocator::init_heap(&mut mapper, &mut frame_allocator)
		.expect("heap initialization failed");

	let mut executor = Executor::new();
    	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();

	mirianu_os::hlt_loop(); 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
	mirianu_os::hlt_loop(); 
}
