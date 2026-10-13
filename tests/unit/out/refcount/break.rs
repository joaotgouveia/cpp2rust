extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_test_0(mut n: i32) -> i32 {
    let mut x: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    'loop_: while (i < n) {
        x += 1;
        if (x == 100) {
            break;
        }
        let mut k: i32 = 0;
        let mut w: i32 = 0;
        'loop_: while (w < j) {
            break;
            {
                {
                    w += 1;
                    k += 1
                };
                i += k
            };
        }
        x = { (x + 1) };
        {
            j = i;
            i += 1
        };
    }
    return x;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ for_test_0(200) }) == 400) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"for_test(200) == 400"),
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
