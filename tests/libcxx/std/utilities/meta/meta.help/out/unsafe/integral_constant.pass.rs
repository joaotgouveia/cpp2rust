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
    const _: () = assert!(true, "_5::value == 5");;
    const _: () = assert!(true, "(std::is_same<_5::value_type, int>::value)");;
    const _: () = assert!(true, "(std::is_same<_5::type, _5>::value)");;
    const _: () = assert!(true, "(_5() == 5)");;
    unimplemented!();
    const _: () = assert!(true, "_5{{}}() == 5");;
    const _: () = assert!(true, "std::true_type{{}}()");;
    const _: () = assert!(true, "std::false_type::value == false");;
    const _: () = assert!(
        true,
        "(std::is_same<std::false_type::value_type, bool>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::false_type::type, std::false_type>::value)"
    );;
    const _: () = assert!(true, "std::true_type::value == true");;
    const _: () = assert!(
        true,
        "(std::is_same<std::true_type::value_type, bool>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::true_type::type, std::true_type>::value)"
    );;
    let mut f1: () = unimplemented!();
    let mut f2: () = f1.clone();
    unimplemented!();
    let mut t1: () = unimplemented!();
    let mut t2: () = t1.clone();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
