#![allow(clippy::empty_loop, unused_labels)]
#![no_main]
#![no_std]

use bootloader_api::info::Optional;
use bootloader_api::BootInfo;

use fixed_vectors::{Vector3, Vector2};
use lazy_static::lazy_static;
use spin::Mutex;

use core::ops::DerefMut;
#[cfg(not(test))]
use core::panic::PanicInfo;

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

    // Easiest way or us to call `writer.write_pixel` from top to down while keeping proper color order.
    let mut y_decrement = resolution.y - 1;

    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let color = Vector3::new(
                x as f64 / (resolution.x - 1) as f64,
                y_decrement as f64 / (resolution.y - 1) as f64,
                0.25,
            );

            let color = color.map(|f| (255.999 * f) as u8);
            let position = Vector2::new(x, y);

            writer.write_pixel(
                position,
                color
            );
        }

        y_decrement -= 1;
    }

    loop {  }
}


bootloader_api::entry_point!(main);