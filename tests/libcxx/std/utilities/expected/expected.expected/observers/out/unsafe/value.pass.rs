extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    {
        let mut e: () = unimplemented!();
        let x: *mut () = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(x), int&>");;
        unimplemented!();
        unimplemented!();
    }
    {
        let e: () = unimplemented!();
        let x: *const () = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(x), const int&>");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut e: () = unimplemented!();
        let x: *mut () = (unsafe { e.value_37() });
        const _: () = assert!(true, "std::same_as<decltype(x), int&&>");;
        unimplemented!();
        unimplemented!();
    }
    {
        let e: () = unimplemented!();
        let x: *const () = (unsafe { e.value_36() });
        const _: () = assert!(true, "std::same_as<decltype(x), const int&&>");;
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn testException_1() {}
pub unsafe fn testAsConst_2() {}
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_1() });
    (unsafe { testAsConst_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
