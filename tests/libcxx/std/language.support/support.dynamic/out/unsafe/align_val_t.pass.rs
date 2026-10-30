extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    const _: () = assert!(true, "std::is_enum<std::align_val_t>::value");;
    const _: () = assert!(
        true,
        "std::is_same<std::underlying_type<std::align_val_t>::type, std::size_t>::value"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible<std::align_val_t, std::size_t>::value"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible<std::size_t, std::align_val_t>::value"
    );;
    {
        let mut a: () = ((0_()) as ());
        let mut b: () = ((32_()) as ());
        let mut c: () = ((-1_()) as ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
