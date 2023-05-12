#![allow(clippy::empty_loop, unused_labels)]
#![no_main]
#![no_std]

#![test_runner(tests::_empty_test_runner)]
#![feature(custom_test_frameworks)]

use bootloader_api::info::Optional;
use bootloader_api::BootInfo;

use lazy_static::lazy_static;
use fixed_vectors::Vector2;
use uart_16550::SerialPort;
use spin::Mutex;

use core::ops::DerefMut;

pub mod raytracer;
pub mod writer;
pub mod macros;
pub mod qemu;

#[cfg(test)]
mod tests;


lazy_static!{
    pub static ref FRAME_BUFFER_WRITER: Mutex<Option<writer::FrameBufferWriter<'static>>> = Mutex::new(
        // The `FRAME_BUFFER_WRITER` needs to be initialized in `main` with the `info.framebuffer`
        None
    );

    pub static ref SERIAL_ONE: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();

        Mutex::new(serial_port)
    };
}


#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}", info);

    loop {  }
}


#[panic_handler]
#[cfg(test)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");

    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);

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

    #[cfg(test)] {
        tests::_testing_main(); loop {  }
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