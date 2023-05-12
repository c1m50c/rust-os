
use lazy_static::lazy_static;
use spin::Lazy;

use crate::qemu::{QemuExitCode, exit_qemu};
use crate::{serial_println, serial_print};


lazy_static!{
    pub static ref TESTS: Lazy<&'static [(&'static str, fn())]> = Lazy::new(|| &[
        ("Basic Assertion", || assert_eq!(2 + 2, 4)),
    ]);
}


#[doc(hidden)]
pub fn _empty_test_runner() { unimplemented!() }


#[doc(hidden)]
pub fn _testing_main() {
    for (name, func) in TESTS.iter() {
        serial_print!("Running `{}`... ", name);
        func();
        serial_println!("[ok]");
    }

    exit_qemu(QemuExitCode::Success);
}