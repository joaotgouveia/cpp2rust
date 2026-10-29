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
    let mut i: () = 0_();
    let mut ri: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(ri), std::reference_wrapper<int>>"
    );;
    let mut ri2: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(ri2), std::reference_wrapper<int>>"
    );;
    let j: () = 0_();
    let mut rj: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(rj), std::reference_wrapper<const int>>"
    );;
    let mut rj2: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<decltype(rj2), std::reference_wrapper<const int>>"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
