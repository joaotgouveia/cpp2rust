extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn loopctl_0() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < 5) as i32) != 0) {
        goto_block!({
            '__entry: {
                if (((i == 1) as i32) != 0) {
                    i.postfix_inc();
                    continue 'loop_;
                }
                if (((i == 4) as i32) != 0) {
                    break;
                }
                goto!('add);
            }
            'add: {
                sum += 1;
            }
        });
        i.postfix_inc();
    }
    return sum;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ loopctl_0() }) == 3) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"loopctl() == 3"),
                        Ptr::<i8>::from_string_literal(b"goto_loop_control.c"),
                        20_u32,
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
