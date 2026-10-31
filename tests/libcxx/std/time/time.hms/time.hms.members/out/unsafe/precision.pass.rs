extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut powers_0: std::cell::LazyCell<[u64; 20]> = std::cell::LazyCell::new(|| unsafe {
    [
        1_u64,
        10_u64,
        100_u64,
        1000_u64,
        10000_u64,
        100000_u64,
        1000000_u64,
        10000000_u64,
        100000000_u64,
        1000000000_u64,
        10000000000_u64,
        100000000000_u64,
        1000000000000_u64,
        10000000000000_u64,
        100000000000000_u64,
        1000000000000000_u64,
        10000000000000000_u64,
        100000000000000000_u64,
        1000000000000000000_u64,
        10000000000000000000_u64,
    ]
});
pub unsafe fn check_precision_1() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_2() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_3() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_4() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_5() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_6() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_7() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_8() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_9() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_10() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_11() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_12() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_13() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_14() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_15() -> bool {
    return unimplemented!();
}
pub unsafe fn check_precision_16() -> bool {
    return unimplemented!();
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const powers_0);
}
