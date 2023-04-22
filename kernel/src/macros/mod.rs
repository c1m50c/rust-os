use core::fmt::{self, Write};
use core::ops::DerefMut;

use crate::FONT_WRITER;


#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => {
        $crate::macros::_print(
            format_args!( $($arg)* )
        );
    };
}


#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };

    ($($arg: tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    }
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut lock = FONT_WRITER.lock();

    if let Some(writer) = lock.deref_mut() {
        writer.write_fmt(args)
            .expect("In `_print()`, the `write_fmt()` function is expected not to fail");
    }
}