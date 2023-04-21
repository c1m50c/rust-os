#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]


use bootloader_api::info::Optional;
use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use core::fmt::Write;

pub mod writer;
pub mod math;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}


fn main(info: &'static mut BootInfo) -> ! {
    if let Optional::Some(buffer) = &mut info.framebuffer {
        let frame_buffer_info = buffer.info();
        let frame_buffer = buffer.buffer_mut();

        let mut font_writer = writer::FontWriter::new(frame_buffer_info, frame_buffer);
        font_writer.clear();

        let _ = write!(&mut font_writer, "Hello, World!\n");
        let _ = write!(&mut font_writer, "https://github.com/c1m50c/rust-os");
    }

    loop {  }
}


bootloader_api::entry_point!(main);