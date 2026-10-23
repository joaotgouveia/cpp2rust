extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_width_0() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_1)) == (0_()));
}
pub unsafe fn check_width_2() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_3)) == (0_()));
}
pub unsafe fn check_width_4() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_5)) == (0_()));
}
pub unsafe fn check_width_6() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_7)) == (3_()));
}
pub unsafe fn check_width_8() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_9)) == (6_()));
}
pub unsafe fn check_width_10() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_11)) == (9_()));
}
pub unsafe fn check_width_12() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_13)) == (1_()));
}
pub unsafe fn check_width_14() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_15)) == (6_()));
}
pub unsafe fn check_width_16() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_17)) == (2_()));
}
pub unsafe fn check_width_18() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_19)) == (1_()));
}
pub unsafe fn check_width_20() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_21)) == (6_()));
}
pub unsafe fn check_width_22() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_23)) == (6_()));
}
pub unsafe fn check_width_24() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_25)) == (3_()));
}
pub unsafe fn check_width_26() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_27)) == (6_()));
}
pub unsafe fn check_width_28() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_29)) == (1_()));
}
pub unsafe fn check_width_30() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut fractional_width_31)) == (4_()));
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
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
pub unsafe fn __cpp2rust_init_globals() {}
