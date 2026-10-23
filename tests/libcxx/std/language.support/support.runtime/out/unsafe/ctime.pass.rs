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
    let mut c: ::libc::clock_t = (0 as ::libc::clock_t);
    let mut s: () = 0_();
    let mut t: ::libc::time_t = (0 as ::libc::time_t);
    let mut tm: ::libc::tm = unsafe { std::mem::zeroed() };
    (&(c));
    (&(s));
    (&(t));
    (&(tm));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::clock()), std::clock_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::difftime(t,t)), double>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::mktime(&tm)), std::time_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::time(&t)), std::time_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::asctime(&tm)), char*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::ctime(&t)), char*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::gmtime(&t)), std::tm*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::localtime(&t)), std::tm*>::value)"
    );;
    let mut c1: *mut () = std::ptr::null_mut();
    let mut c2: *const () = std::ptr::null();
    (&(c1));
    (&(c2));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::strftime(c1,s,c2,&tm)), std::size_t>::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
