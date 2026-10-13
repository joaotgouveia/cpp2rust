extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn run_0() -> i32 {
    let mut i: i32 = 0;
    let mut runs: i32 = 0;
    let mut __do_while = true;
    'loop_: while __do_while || (((i < 4) as i32) != 0) {
        __do_while = false;
        runs += 1;
        i += 1;
        if (((i == 4) as i32) != 0) {
            continue 'loop_;
        }
    }
    return runs;
}
pub fn nested_1() -> i32 {
    let mut oi: i32 = 0;
    let mut runs: i32 = 0;
    let mut __do_while = true;
    'loop_: while __do_while || (((oi < 2) as i32) != 0) {
        __do_while = false;
        oi += 1;
        let mut ii: i32 = 0;
        let mut __do_while = true;
        'loop_: while __do_while || (((ii < 3) as i32) != 0) {
            __do_while = false;
            runs += 1;
            ii += 1;
            if (((ii == 3) as i32) != 0) {
                continue 'loop_;
            }
        }
    }
    return runs;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ run_0() }) == 4) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"run() == 4"),
                        Ptr::<i8>::from_string_literal(b"do_while_continue.c"),
                        34_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ nested_1() }) == 6) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"nested() == 6"),
                        Ptr::<i8>::from_string_literal(b"do_while_continue.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
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
