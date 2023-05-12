use core::fmt::{self, Write};

use crate::SERIAL_ONE;


#[macro_export]
macro_rules! serial_print {
    ($($arg: tt)*) => {
        $crate::macros::serial::_print(
            format_args!( $($arg)* )
        );
    };
}


#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n");
    };

    ($($arg: tt)*) => {
        $crate::serial_print!("{}\n", format_args!($($arg)*));
    }
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut lock = SERIAL_ONE.lock();

    lock.write_fmt(args)
        .expect("In `_print()`, the `write_fmt()` function is expected not to fail");
}