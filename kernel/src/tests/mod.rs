
use lazy_static::lazy_static;
use spin::Lazy;

use crate::{println, print};


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
        print!("Running `{}`... ", name);
        func();
        println!("[ok]");
    }
}