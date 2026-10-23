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
    let mut ia: [(); 10] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_(), 10_()];
    let mut ia1: [(); 10] = [2_(), 7_(), 1_(), 4_(), 3_(), 6_(), 5_(), 10_(), 9_(), 8_()];
    let mut ia2: [(); 10] = [1_(), 8_(), 3_(), 4_(), 6_(), 9_(), 5_(), 7_(), 2_(), 10_()];
    let sa: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut g: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
