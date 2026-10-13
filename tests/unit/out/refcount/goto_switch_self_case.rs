extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(mut n: i32) -> i32 {
    let mut steps: i32 = 0;
    switch!(match n {
        __v if __v == 0 => 'target: {
            steps += 1;
            break;
        }
        __v if __v == 1 => {
            steps += 10;
            goto!('target);
        }
        _ => {
            steps = -1_i32;
            break;
        }
    });
    return steps;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ sm_0(0) }) == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"sm(0) == 1"),
                        Ptr::<i8>::from_string_literal(b"goto_switch_self_case.c"),
                        21_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ sm_0(1) }) == 11) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"sm(1) == 11"),
                        Ptr::<i8>::from_string_literal(b"goto_switch_self_case.c"),
                        22_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ sm_0(7) }) == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"sm(7) == -1"),
                        Ptr::<i8>::from_string_literal(b"goto_switch_self_case.c"),
                        23_u32,
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
