extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn retry_0(mut n: i32) -> i32 {
    let mut count: i32 = 0_i32;
    let mut acc: i32 = 0_i32;
    goto_block!({
        '__entry: {
            count = 0;
            acc = 0;
        }
        'again: {
            count += 1;
            acc += n;
            if (((count < 3) as i32) != 0) {
                goto!('again);
            }
            return acc;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ retry_0(4) }) == 12) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"retry(4) == 12"),
                        Ptr::<i8>::from_string_literal(b"goto_backward.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
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
