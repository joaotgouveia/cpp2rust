extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoAssign {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DieOnAssign_0_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DieOnAssign_1_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DieOnAssign_2_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DieOnAssign_3_ {}
pub unsafe fn test_arity_checks_0() {
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P const&>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, A const&>::value");;
    }
}
pub unsafe fn test_assignability_checks_1() {
    {
        const _: () = assert!(true, "!std::is_assignable<T1&, T2 const&>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T1&, T2 const&>::value");;
    }
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
    (unsafe { test_arity_checks_0() });
    (unsafe { test_assignability_checks_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
