extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (3499211612_())));
    assert!(((unimplemented!()) == (581869302_())));
    assert!(((unimplemented!()) == (3890346734_())));
}
pub unsafe fn test2_1() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!() as u64) == (14514284786278117030_u64)));
    assert!(((unimplemented!() as u64) == (4620546740167642908_u64)));
    assert!(((unimplemented!() as u64) == (13109570281517897720_u64)));
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
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
