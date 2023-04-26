#![allow(clippy::empty_loop, unused_labels)]
#![no_main]
#![no_std]

use bootloader_api::info::Optional;
use bootloader_api::BootInfo;

use lazy_static::lazy_static;
use fixed_vectors::Vector2;
use spin::Mutex;

use core::ops::DerefMut;
#[cfg(not(test))]
use core::panic::PanicInfo;

pub mod raytracer;
pub mod writer;
pub mod macros;


lazy_static!{
    pub static ref FRAME_BUFFER_WRITER: Mutex<Option<writer::FrameBufferWriter<'static>>> = Mutex::new(
        // The `FRAME_BUFFER_WRITER` needs to be initialized in `main` with the `info.framebuffer`
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
    // Initialize `FRAME_BUFFER_WRITER` with `info.framebuffer`
    if let Optional::Some(buffer) = &mut info.framebuffer {
        let frame_buffer_info = buffer.info();
        let frame_buffer = buffer.buffer_mut();

        let mut lock = FRAME_BUFFER_WRITER.lock();

        let mut writer = writer::FrameBufferWriter::new(frame_buffer_info, frame_buffer);
        writer.clear();

        *lock = Some(writer);
    }

    let mut lock = FRAME_BUFFER_WRITER.lock();
    let Some(writer) = lock.deref_mut() else { unreachable!() };

    let resolution = Vector2::new(
        writer.frame_buffer_info.width, writer.frame_buffer_info.height
    );

    let mut raytracer = raytracer::Raytracer::new(resolution, writer);
    raytracer.run();

    loop {  }
}


bootloader_api::entry_point!(main);