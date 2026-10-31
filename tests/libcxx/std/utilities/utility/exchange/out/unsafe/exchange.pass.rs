extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() -> bool {
    let mut v: i32 = 12;
    if ((12) != (unimplemented!())) || ((v) != (23)) {
        return false;
    }
    if ((23) != (unimplemented!())) || ((v) != (67)) {
        return false;
    }
    if ((67) != (unimplemented!())) || ((v) != (0)) {
        return false;
    }
    return true;
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TestNoexcept_true__true_ {}
impl TestNoexcept_true__true_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TestNoexcept_true__false_ {}
impl TestNoexcept_true__false_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TestNoexcept_false__true_ {}
impl TestNoexcept_false__true_ {}
pub unsafe fn test_noexcept_1() -> bool {
    {
        let mut x: i32 = 42;
        const _: () = assert!(true, "");;
    }
    {
        let mut x: TestNoexcept_true__true_ = <TestNoexcept_true__true_>::default();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut x: TestNoexcept_true__false_ = <TestNoexcept_true__false_>::default();
        const _: () = assert!(true, "");;
    }
    {
        let mut x: TestNoexcept_false__true_ = <TestNoexcept_false__true_>::default();
        const _: () = assert!(true, "");;
    }
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
    {
        let mut v: i32 = 12;
        assert!(((unimplemented!()) == (12)));
        assert!(((v) == (23)));
        assert!(((unimplemented!()) == (23)));
        assert!(((v) == (67)));
        assert!(((unimplemented!()) == (67)));
        assert!(((v) == (0)));
    }
    {
        let mut b: bool = false;
        assert!(!(unimplemented!()));
        assert!(b);
    }
    {
        let s1: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"Hi Mom!".as_ptr(),
            libcc2rs::c_str_len(c"Hi Mom!".as_ptr()) + 1,
        )
        .to_vec();
        let s2: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"Yo Dad!".as_ptr(),
            libcc2rs::c_str_len(c"Yo Dad!".as_ptr()) + 1,
        )
        .to_vec();
        let mut s3: Vec<libc::c_char> = s1.clone();
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        s3 = s2.clone();
        assert!(unimplemented!());
        assert!(((s3.len() - 1) == (0_usize)));
        s3 = s2.clone();
        assert!(unimplemented!());
        assert!(((s3.len() - 1) == (0_usize)));
    }
    const _: () = assert!(true, "test_constexpr()");;
    const _: () = assert!(true, "test_noexcept()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
