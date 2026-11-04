extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 1;
    {
        let mut x: i32 = 2;
        ({ cpp2rust_assert_fail_0((x == 2)) });
        {
            let mut x: i32 = 3;
            ({ cpp2rust_assert_fail_0((x == 3)) });
        }
        ({ cpp2rust_assert_fail_0((x == 2)) });
    }
    ({ cpp2rust_assert_fail_0((x == 1)) });
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        let mut y: i32 = i;
        {
            let mut y: i32 = 10;
            sum += y;
        }
        sum += y;
        i.postfix_inc();
    }
    ({ cpp2rust_assert_fail_0((sum == 33)) });
    if (x == 1) {
        let mut x: i32 = 5;
        {
            let mut x: i32 = 6;
            ({ cpp2rust_assert_fail_0((x == 6)) });
        }
        ({ cpp2rust_assert_fail_0((x == 5)) });
    }
    ({ cpp2rust_assert_fail_0((x == 1)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
