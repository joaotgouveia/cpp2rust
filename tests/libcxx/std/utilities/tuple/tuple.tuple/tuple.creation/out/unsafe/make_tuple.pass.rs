extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    let mut i: () = 0_();
    let mut j: () = 0_();
    let mut t: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    i = 2_();
    j = 3.5_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 0_();
    (*unimplemented!()) = 0_();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t1: () = unimplemented!();
        let i1: () = (*unimplemented!());
        let d1: () = (*unimplemented!());
        const _: () = assert!(true, "i1 == 1");;
        const _: () = assert!(true, "d1 == 3.14");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
