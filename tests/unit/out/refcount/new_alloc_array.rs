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
    let mut array: Ptr<i32> = Ptr::alloc_array((0..100_u64).map(|_| 0_i32).collect::<Box<[i32]>>());
    ({
        let ___s: AnyPtr = (array).to_any();
        memset_0(
            ___s,
            0,
            ((::std::mem::size_of::<i32>() as u64).wrapping_mul(100_u64) as u64),
        )
    });
    elem!(array, 99).write(-1_i32);
    let mut p1: Ptr<i32> = (array).clone();
    'loop_: while ((p1.read()) >= 0) {
        p1.write(1);
        p1.prefix_inc();
    }
    let mut out: i32 = 0;
    let mut p1: Ptr<i32> = (array).clone();
    'loop_: while ((p1.read()) >= 0) {
        out += { (p1.read()) };
        p1.prefix_inc();
    }
    let mut p2: Ptr<i32> = (array).clone();
    p2.delete();
    ({ cpp2rust_assert_fail_1((out == 99)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
