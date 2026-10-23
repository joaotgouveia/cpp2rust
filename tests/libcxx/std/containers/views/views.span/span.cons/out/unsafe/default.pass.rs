extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn checkCV_0() {
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testConstexprSpan_1() -> () {
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()).is_null()) && ((unimplemented!()) == (0_())))
        && ((unimplemented!()).is_null()))
        && ((unimplemented!()) == (0_()));
}
pub unsafe fn testConstexprSpan_2() -> () {
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()).is_null()) && ((unimplemented!()) == (0_())))
        && ((unimplemented!()).is_null()))
        && ((unimplemented!()) == (0_()));
}
pub unsafe fn testConstexprSpan_3() -> () {
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()).is_null()) && ((unimplemented!()) == (0_())))
        && ((unimplemented!()).is_null()))
        && ((unimplemented!()) == (0_()));
}
pub unsafe fn testConstexprSpan_4() -> () {
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()).is_null()) && ((unimplemented!()) == (0_())))
        && ((unimplemented!()).is_null()))
        && ((unimplemented!()) == (0_()));
}
pub unsafe fn testRuntimeSpan_5() {
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_6() {
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_7() {
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_8() {
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_9() {
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
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
    const _: () = assert!(true, "testConstexprSpan<int>()");;
    const _: () = assert!(true, "testConstexprSpan<long>()");;
    const _: () = assert!(true, "testConstexprSpan<double>()");;
    const _: () = assert!(true, "testConstexprSpan<A>()");;
    (unsafe { testRuntimeSpan_5() });
    (unsafe { testRuntimeSpan_6() });
    (unsafe { testRuntimeSpan_7() });
    (unsafe { testRuntimeSpan_8() });
    (unsafe { testRuntimeSpan_9() });
    (unsafe { checkCV_0() });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<std::span<int, std::dynamic_extent>>"
    );;
    const _: () = assert!(true, "std::is_default_constructible_v<std::span<int, 0>>");;
    const _: () = assert!(true, "!std::is_default_constructible_v<std::span<int, 2>>");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
