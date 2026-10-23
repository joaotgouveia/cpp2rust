extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_width_0() -> () {
    return (fractional_width_1.with(|rc| *rc.borrow()) == 0_());
}
pub fn check_width_2() -> () {
    return (fractional_width_3.with(|rc| *rc.borrow()) == 0_());
}
pub fn check_width_4() -> () {
    return (fractional_width_5.with(|rc| *rc.borrow()) == 0_());
}
pub fn check_width_6() -> () {
    return (fractional_width_7.with(|rc| *rc.borrow()) == 3_());
}
pub fn check_width_8() -> () {
    return (fractional_width_9.with(|rc| *rc.borrow()) == 6_());
}
pub fn check_width_10() -> () {
    return (fractional_width_11.with(|rc| *rc.borrow()) == 9_());
}
pub fn check_width_12() -> () {
    return (fractional_width_13.with(|rc| *rc.borrow()) == 1_());
}
pub fn check_width_14() -> () {
    return (fractional_width_15.with(|rc| *rc.borrow()) == 6_());
}
pub fn check_width_16() -> () {
    return (fractional_width_17.with(|rc| *rc.borrow()) == 2_());
}
pub fn check_width_18() -> () {
    return (fractional_width_19.with(|rc| *rc.borrow()) == 1_());
}
pub fn check_width_20() -> () {
    return (fractional_width_21.with(|rc| *rc.borrow()) == 6_());
}
pub fn check_width_22() -> () {
    return (fractional_width_23.with(|rc| *rc.borrow()) == 6_());
}
pub fn check_width_24() -> () {
    return (fractional_width_25.with(|rc| *rc.borrow()) == 3_());
}
pub fn check_width_26() -> () {
    return (fractional_width_27.with(|rc| *rc.borrow()) == 6_());
}
pub fn check_width_28() -> () {
    return (fractional_width_29.with(|rc| *rc.borrow()) == 1_());
}
pub fn check_width_30() -> () {
    return (fractional_width_31.with(|rc| *rc.borrow()) == 4_());
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
        "check_width<std::chrono::hours,                               0>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::minutes,                             0>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::seconds,                             0>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::milliseconds,                        3>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::microseconds,                        6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::nanoseconds,                         9>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   2>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   3>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   4>>, 2>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   5>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   6>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   7>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   8>>, 3>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,   9>>, 6>()"
    );;
    const _: () = assert!(
        true,
        "check_width<std::chrono::duration<int, std::ratio<  1,  10>>, 1>()"
    );;
    const _: () = assert!(
        true,
        "check_width<microfortnights,                                  4>()"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
