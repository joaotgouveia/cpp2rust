extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_double_ended_contiguous_container_asan_correct_0(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn test_1(mut c: ()) {
    let mut it1: () = unimplemented!();
    let mut it2: () = unimplemented!();
    unimplemented!();
    let mut it3: () = unimplemented!();
    let mut it4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    let mut queue: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (4098_())) {
        unimplemented!();
        i.prefix_inc();
    }
    'loop_: while ((unimplemented!()) > (1_())) {
        (unsafe { test_1(unimplemented!()) });
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
