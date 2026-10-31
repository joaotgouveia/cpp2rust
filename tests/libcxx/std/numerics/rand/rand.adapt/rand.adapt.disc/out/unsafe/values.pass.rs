extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn where_0(_a0: *const u64) {}
pub unsafe fn test1_1() {
    const _: () = assert!(true, "(E::block_size == 223)");;
    const _: () = assert!(true, "(E::used_block == 23)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFF)");;
    (unsafe { where_0(&*(&raw const unimplemented!()).cast::<u64>()) });
    (unsafe { where_0(&*(&raw const unimplemented!()).cast::<u64>()) });
}
pub unsafe fn test2_2() {
    const _: () = assert!(true, "(E::block_size == 389)");;
    const _: () = assert!(true, "(E::used_block == 11)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFFFFFFFFull)");;
    (unsafe { where_0(&*(&raw const unimplemented!()).cast::<u64>()) });
    (unsafe { where_0(&*(&raw const unimplemented!()).cast::<u64>()) });
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
    (unsafe { test1_1() });
    (unsafe { test2_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
