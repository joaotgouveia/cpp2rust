extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() -> () {
    let mut v: () = 12_();
    if ((12_()) != (unimplemented!())) || ((v) != (23_())) {
        return false;
    }
    if ((23_()) != (unimplemented!())) || ((v) != (67_())) {
        return false;
    }
    if ((67_())
        != (unsafe {
            let mut ___new_value: () = unimplemented!();
            exchange_1(&mut v, &mut ___new_value)
        }))
        || ((v) != (0_()))
    {
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
pub unsafe fn test_noexcept_2() -> () {
    {
        let mut x: () = 42_();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut v: () = 12_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut b: () = false;
        unimplemented!();
        unimplemented!();
    }
    {
        let s1: () = unimplemented!();
        let s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    const _: () = assert!(true, "test_noexcept()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
