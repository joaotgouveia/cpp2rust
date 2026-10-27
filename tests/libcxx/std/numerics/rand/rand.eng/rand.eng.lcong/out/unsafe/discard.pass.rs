extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn rand0_0() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn rand0_1() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn rand0_2() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn rand_3() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn rand_4() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn rand_5() {
    let mut e: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn other_6() {
    let mut e1: () = unimplemented!();
    let mut e2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    &(unimplemented!());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    unimplemented!();
}
pub unsafe fn other_7() {
    let mut e1: () = unimplemented!();
    let mut e2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    &(unimplemented!());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    unimplemented!();
}
pub unsafe fn other_8() {
    let mut e1: () = unimplemented!();
    let mut e2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    &(unimplemented!());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
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
    (unsafe { rand0_0() });
    (unsafe { rand0_1() });
    (unsafe { rand0_2() });
    (unsafe { rand_3() });
    (unsafe { rand_4() });
    (unsafe { rand_5() });
    (unsafe { other_6() });
    (unsafe { other_7() });
    (unsafe { other_8() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
