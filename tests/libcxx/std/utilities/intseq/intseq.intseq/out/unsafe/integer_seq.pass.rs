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
    const _: () = assert!(true, "std::is_same<int3::value_type, int>::value");;
    const _: () = assert!(true, "int3::size() == 3");;
    const _: () = assert!(true, "std::is_same<size1::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size1::size() == 1");;
    const _: () = assert!(
        true,
        "std::is_same<ushort2::value_type, unsigned short>::value"
    );;
    const _: () = assert!(true, "ushort2::size() == 2");;
    const _: () = assert!(true, "std::is_same<bool0::value_type, bool>::value");;
    const _: () = assert!(true, "bool0::size() == 0");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
