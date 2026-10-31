extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn swap_0(a: *mut S, b: *mut S) {
    (*(*a).calls) += 1;
    (*(*b).calls) += 1;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub calls: *mut i32,
}
pub unsafe fn swap_1(a: *const CS, b: *const CS) {
    (*(*a).calls) += 1;
    (*(*b).calls) += 1;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CS {
    pub calls: *mut i32,
}
const _: () = assert!(true, "std::is_swappable_v<std::tuple<>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<S>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<CS>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<S&>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<CS, S>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<CS, S&>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<>>");
const _: () = assert!(true, "!std::is_swappable_v<const std::tuple<S>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<CS>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<S&>>");
const _: () = assert!(true, "!std::is_swappable_v<const std::tuple<CS, S>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<CS, S&>>");
pub unsafe fn test_2() -> bool {
    let mut cs_calls: i32 = 0;
    let mut s_calls: i32 = 0;
    let mut s1: S = S {
        calls: (&mut s_calls as *mut i32),
    };
    let mut s2: S = S {
        calls: (&mut s_calls as *mut i32),
    };
    let t1: () = unimplemented!();
    let t2: () = unimplemented!();
    unimplemented!();
    assert!(((cs_calls) == (2)));
    assert!(((s_calls) == (2)));
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
