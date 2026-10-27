extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static powers_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        10_(),
        100_(),
        1000_(),
        10000_(),
        100000_(),
        1000000_(),
        10000000_(),
        100000000_(),
        1000000000_(),
        10000000000_(),
        100000000000_(),
        1000000000000_(),
        10000000000000_(),
        100000000000000_(),
        1000000000000000_(),
        10000000000000000_(),
        100000000000000000_(),
        1000000000000000000_(),
        10000000000000000000_(),
    ])));
);
pub fn check_precision_1() -> () {
    return is_same_v_2.with(|rc| *rc.borrow());
}
pub fn check_precision_3() -> () {
    return is_same_v_2.with(|rc| *rc.borrow());
}
pub fn check_precision_4() -> () {
    return is_same_v_2.with(|rc| *rc.borrow());
}
pub fn check_precision_5() -> () {
    return is_same_v_6.with(|rc| *rc.borrow());
}
pub fn check_precision_7() -> () {
    return is_same_v_8.with(|rc| *rc.borrow());
}
pub fn check_precision_9() -> () {
    return is_same_v_10.with(|rc| *rc.borrow());
}
pub fn check_precision_11() -> () {
    return is_same_v_12.with(|rc| *rc.borrow());
}
pub fn check_precision_13() -> () {
    return is_same_v_8.with(|rc| *rc.borrow());
}
pub fn check_precision_14() -> () {
    return is_same_v_15.with(|rc| *rc.borrow());
}
pub fn check_precision_16() -> () {
    return is_same_v_12.with(|rc| *rc.borrow());
}
pub fn check_precision_17() -> () {
    return is_same_v_8.with(|rc| *rc.borrow());
}
pub fn check_precision_18() -> () {
    return is_same_v_8.with(|rc| *rc.borrow());
}
pub fn check_precision_19() -> () {
    return is_same_v_6.with(|rc| *rc.borrow());
}
pub fn check_precision_20() -> () {
    return is_same_v_8.with(|rc| *rc.borrow());
}
pub fn check_precision_21() -> () {
    return is_same_v_12.with(|rc| *rc.borrow());
}
pub fn check_precision_22() -> () {
    return is_same_v_23.with(|rc| *rc.borrow());
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    const _: () = assert!(
        true,
        "check_precision<std::chrono::hours,                               0>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::minutes,                             0>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::seconds,                             0>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::milliseconds,                        3>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::microseconds,                        6>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::nanoseconds,                         9>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   2>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   3>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   4>>, 2>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   5>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   6>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   7>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   8>>, 3>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,   9>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<std::chrono::duration<int, std::ratio<  1,  10>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_precision<microfortnights,                                  4>()"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = powers_0.with(|_| ());
}
