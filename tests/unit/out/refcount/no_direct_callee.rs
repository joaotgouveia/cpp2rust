extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() -> bool {
    return false;
}
pub fn test_1(mut fn_: FnPtr<fn() -> bool>) -> i32 {
    if !({ fn_.call() }) {
        return 1;
    }
    return 0;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ test_1(FnPtr::<fn() -> bool>::new(test1_0)) }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"test(test1) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
