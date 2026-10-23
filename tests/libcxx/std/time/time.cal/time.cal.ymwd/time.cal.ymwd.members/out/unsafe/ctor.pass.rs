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
    let January: () = unimplemented!();
    let Tuesday: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let ym0: () = unimplemented!();
    const _: () = assert!(true, "ym0.year()            == year{{}}");;
    const _: () = assert!(true, "ym0.month()           == month{{}}");;
    const _: () = assert!(true, "ym0.weekday()         == weekday{{}}");;
    const _: () = assert!(true, "ym0.index()           == 0");;
    const _: () = assert!(true, "ym0.weekday_indexed() == weekday_indexed{{}}");;
    const _: () = assert!(true, "!ym0.ok()");;
    let ym1: () = unimplemented!();
    const _: () = assert!(true, "ym1.year()            == year{{2019}}");;
    const _: () = assert!(true, "ym1.month()           == January");;
    const _: () = assert!(true, "ym1.weekday()         == Tuesday");;
    const _: () = assert!(true, "ym1.index()           == 1");;
    const _: () = assert!(
        true,
        "ym1.weekday_indexed() == weekday_indexed{{Tuesday, 1}}"
    );;
    const _: () = assert!(true, "ym1.ok()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
