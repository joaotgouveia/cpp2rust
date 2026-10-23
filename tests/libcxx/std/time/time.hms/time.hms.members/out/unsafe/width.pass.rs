extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_width_0() -> () {
    return ((unimplemented!()) == (0_()));
}
pub unsafe fn check_width_1() -> () {
    return ((unimplemented!()) == (0_()));
}
pub unsafe fn check_width_2() -> () {
    return ((unimplemented!()) == (0_()));
}
pub unsafe fn check_width_3() -> () {
    return ((unimplemented!()) == (3_()));
}
pub unsafe fn check_width_4() -> () {
    return ((unimplemented!()) == (6_()));
}
pub unsafe fn check_width_5() -> () {
    return ((unimplemented!()) == (9_()));
}
pub unsafe fn check_width_6() -> () {
    return ((unimplemented!()) == (1_()));
}
pub unsafe fn check_width_7() -> () {
    return ((unimplemented!()) == (6_()));
}
pub unsafe fn check_width_8() -> () {
    return ((unimplemented!()) == (2_()));
}
pub unsafe fn check_width_9() -> () {
    return ((unimplemented!()) == (1_()));
}
pub unsafe fn check_width_10() -> () {
    return ((unimplemented!()) == (6_()));
}
pub unsafe fn check_width_11() -> () {
    return ((unimplemented!()) == (6_()));
}
pub unsafe fn check_width_12() -> () {
    return ((unimplemented!()) == (3_()));
}
pub unsafe fn check_width_13() -> () {
    return ((unimplemented!()) == (6_()));
}
pub unsafe fn check_width_14() -> () {
    return ((unimplemented!()) == (1_()));
}
pub unsafe fn check_width_15() -> () {
    return ((unimplemented!()) == (4_()));
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
