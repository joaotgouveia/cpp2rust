extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    let from: [(); 10] = [
        115 as (), 111 as (), 109 as (), 101 as (), 32 as (), 116 as (), 101 as (), 120 as (),
        116 as (), 0 as (),
    ];
    let mut to: [(); 9] = [unimplemented!(); 9];
    let f: *const () = (unsafe { use_facet_0(unimplemented!()) });
    let mut mbs: () = unimplemented!();
    let mut from_next: *const () = std::ptr::null();
    let mut to_next: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (9_())) {
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
