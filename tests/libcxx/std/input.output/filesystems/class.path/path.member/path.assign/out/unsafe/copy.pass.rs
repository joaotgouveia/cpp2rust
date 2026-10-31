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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    const _: () = assert!(true, "std::is_copy_assignable<path>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable<path>::value");;
    let s: Vec<libc::c_char> =
        std::slice::from_raw_parts(c"foo".as_ptr(), libcc2rs::c_str_len(c"foo".as_ptr()) + 1)
            .to_vec();
    let p: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = s
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let mut p2: Vec<libc::c_char> = unimplemented!();
    let pref: *mut Vec<libc::c_char> = &mut (*unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((pref) == (&mut p2 as *mut Vec<libc::c_char>)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
