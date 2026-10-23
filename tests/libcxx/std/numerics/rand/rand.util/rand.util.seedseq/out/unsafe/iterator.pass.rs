extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    {
        let mut a: [(); 5] = [5_(), 4_(), 3_(), 2_(), 1_()];
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut b: [(); 5] = [
            0_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 4] = [
            1311673392922361856_(),
            78187493376_(),
            18446744073709551615_(),
            6442450944_(),
        ];
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut b: [(); 4] = [0_(), unimplemented!(), unimplemented!(), unimplemented!()];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        let mut s: () = unimplemented!();
        unimplemented!();
        let mut b: [(); 3] = [0_(), unimplemented!(), unimplemented!()];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
