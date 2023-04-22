#![allow(clippy::empty_loop, unused_labels)]
#![no_main]
#![no_std]

use bootloader_api::info::Optional;
use bootloader_api::BootInfo;

use lazy_static::lazy_static;
use spin::Mutex;

#[cfg(not(test))]
use core::panic::PanicInfo;

pub mod writer;
pub mod macros;
pub mod math;


lazy_static!{
    pub static ref FONT_WRITER: Mutex<Option<writer::FontWriter<'static>>> = Mutex::new(
        // The `FONT_WRITER` needs to be initialized in `main` with the `info.framebuffer`
        None
    );
}


#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);

    loop {  }
}


fn main(info: &'static mut BootInfo) -> ! {
    // Initialize `FONT_WRITER` with `info.framebuffer`
    if let Optional::Some(buffer) = &mut info.framebuffer {
        let frame_buffer_info = buffer.info();
        let frame_buffer = buffer.buffer_mut();

        let mut lock = FONT_WRITER.lock();

        let mut writer = writer::FontWriter::new(frame_buffer_info, frame_buffer);
        writer.clear();

        *lock = Some(writer);
    }

    println!("Hello, World!");

    loop {  }
}


bootloader_api::entry_point!(main);