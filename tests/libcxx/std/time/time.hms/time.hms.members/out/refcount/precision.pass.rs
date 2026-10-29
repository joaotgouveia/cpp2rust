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
    return unimplemented!();
}
pub fn check_precision_2() -> () {
    return unimplemented!();
}
pub fn check_precision_3() -> () {
    return unimplemented!();
}
pub fn check_precision_4() -> () {
    return unimplemented!();
}
pub fn check_precision_5() -> () {
    return unimplemented!();
}
pub fn check_precision_6() -> () {
    return unimplemented!();
}
pub fn check_precision_7() -> () {
    return unimplemented!();
}
pub fn check_precision_8() -> () {
    return unimplemented!();
}
pub fn check_precision_9() -> () {
    return unimplemented!();
}
pub fn check_precision_10() -> () {
    return unimplemented!();
}
pub fn check_precision_11() -> () {
    return unimplemented!();
}
pub fn check_precision_12() -> () {
    return unimplemented!();
}
pub fn check_precision_13() -> () {
    return unimplemented!();
}
pub fn check_precision_14() -> () {
    return unimplemented!();
}
pub fn check_precision_15() -> () {
    return unimplemented!();
}
pub fn check_precision_16() -> () {
    return unimplemented!();
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
