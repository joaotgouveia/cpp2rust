extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cases_and_default_stacked_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 42;
                break 'switch;
            }
        }
    };
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ cases_and_default_stacked_0(1) }) == 42) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"cases_and_default_stacked(1) == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ cases_and_default_stacked_0(2) }) == 42) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"cases_and_default_stacked(2) == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ cases_and_default_stacked_0(3) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"cases_and_default_stacked(3) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ cases_and_default_stacked_0(99) }) == 42) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"cases_and_default_stacked(99) == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
