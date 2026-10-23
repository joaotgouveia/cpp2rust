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
    const _: () = assert!(
        true,
        "(std::is_base_of<std::locale::facet, std::time_get<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_base_of<std::locale::facet, std::time_get<wchar_t> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_base_of<std::time_base, std::time_get<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_base_of<std::time_base, std::time_get<wchar_t> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::time_get<char>::char_type, char>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::time_get<wchar_t>::char_type, wchar_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::time_get<char>::iter_type, std::istreambuf_iterator<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::time_get<wchar_t>::iter_type, std::istreambuf_iterator<wchar_t> >::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
