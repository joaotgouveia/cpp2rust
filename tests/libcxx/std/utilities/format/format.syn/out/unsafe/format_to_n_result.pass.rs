extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut v: () = unimplemented!();
    let mut __decomp_1: () = v.clone();
    const _: () = assert!(true, "std::same_as<decltype(out), CharT*>");;
    assert!(((__decomp_1.out) == (v.out)));
    const _: () = assert!(
        true,
        "std::same_as<decltype(size), std::iter_difference_t<CharT*>>"
    );;
    assert!(((__decomp_1.size) == (v.size)));
}
pub unsafe fn test_2() {
    let mut v: () = unimplemented!();
    let mut __decomp_3: () = v.clone();
    const _: () = assert!(true, "std::same_as<decltype(out), CharT*>");;
    assert!(((__decomp_3.out) == (v.out)));
    const _: () = assert!(
        true,
        "std::same_as<decltype(size), std::iter_difference_t<CharT*>>"
    );;
    assert!(((__decomp_3.size) == (v.size)));
}
pub unsafe fn test_4() -> bool {
    (unsafe { test_0() });
    (unsafe { test_2() });
    return true;
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
    assert!((unsafe { test_4() }));
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
