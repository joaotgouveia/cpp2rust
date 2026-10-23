extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut powers_0: std::cell::LazyCell<[(); 20]> = std::cell::LazyCell::new(|| unsafe {
    [
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
    ]
});
pub unsafe fn check_precision_1() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_2));
}
pub unsafe fn check_precision_3() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_2));
}
pub unsafe fn check_precision_4() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_2));
}
pub unsafe fn check_precision_5() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_6));
}
pub unsafe fn check_precision_7() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8));
}
pub unsafe fn check_precision_9() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_10));
}
pub unsafe fn check_precision_11() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_12));
}
pub unsafe fn check_precision_13() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8));
}
pub unsafe fn check_precision_14() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_15));
}
pub unsafe fn check_precision_16() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_12));
}
pub unsafe fn check_precision_17() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8));
}
pub unsafe fn check_precision_18() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8));
}
pub unsafe fn check_precision_19() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_6));
}
pub unsafe fn check_precision_20() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8));
}
pub unsafe fn check_precision_21() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_12));
}
pub unsafe fn check_precision_22() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_23));
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
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const powers_0);
}
