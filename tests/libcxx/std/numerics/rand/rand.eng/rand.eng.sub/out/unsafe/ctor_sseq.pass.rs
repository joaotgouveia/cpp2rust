extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0() {
    let mut a : *const () = c"13604817 711567 9760686 13278398 3323440 175548 5553651 3028863 10748297 2216688 275779 14778841 14438394 9483441 4229545 14657301 12636508 15978210 1653340 1718567 9272421 14302862 7940348 889045 0" .as_ptr() ;
    let mut as_: [(); 3] = [3_(), 5_(), 7_()];
    let mut sseq: () = unimplemented!();
    let mut e1: () = unimplemented!();
    let mut os: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test2_1() {
    let mut a : *const () = c"241408498702289 172342669275054 191026374555184 61020585639411 231929771458953 142769679250755 198672786411514 183712717244841 227473912549724 62843577252444 68782400568421 159248704678140 0" .as_ptr() ;
    let mut as_: [(); 3] = [3_(), 5_(), 7_()];
    let mut sseq: () = unimplemented!();
    let mut e1: () = unimplemented!();
    let mut os: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
