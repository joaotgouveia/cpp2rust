extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut v: i32) -> i32 {
    return (v * 2);
}
pub fn switch_on_call_1(mut x: i32) -> i32 {
    'switch: {
        match { ({ double_it_0(x) }) } {
            __v if __v == 0 => {
                return 100;
            }
            __v if __v == 2 => {
                return 200;
            }
            __v if __v == 4 => {
                return 400;
            }
            _ => {
                return -1_i32;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ switch_on_call_1(0) }) == 100) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"switch_on_call(0) == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ switch_on_call_1(1) }) == 200) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"switch_on_call(1) == 200"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ switch_on_call_1(2) }) == 400) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"switch_on_call(2) == 400"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ switch_on_call_1(99) }) == -1_i32) {
        &(0);
    } else {
        ({
            __assert_fail_2(
                Ptr::<i8>::from_string_literal(b"switch_on_call(99) == -1"),
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
