extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn scan_0(mut n: i32) -> i32 {
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < n) as i32) != 0) {
        let mut j: i32 = 0_i32;
        goto_block!({
            '__entry: {
                j = 0;
                'loop_: while (((j < 10) as i32) != 0) {
                    if (((j == 5) as i32) != 0) {
                        goto!('next);
                    }
                    total += 1;
                    j.postfix_inc();
                }
                total += 100;
            }
            'next: {
                total += 1000;
            }
        });
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ scan_0(2) }) == 2010) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"scan(2) == 2010"),
                        Ptr::<i8>::from_string_literal(b"goto_nested_label.c"),
                        22_u32,
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
