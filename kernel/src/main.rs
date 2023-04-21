#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]


use bootloader_api::BootInfo;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}


fn main(_: &'static mut BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let text = b"Hello, World!".iter();

    for (idx, byte) in text.enumerate() {
        unsafe {
            *vga_buffer.offset(idx as isize * 2) = *byte;
            *vga_buffer.offset(idx as isize * 2 + 1) = 0xb;
        }
    }

    loop {  }
}


bootloader_api::entry_point!(main);