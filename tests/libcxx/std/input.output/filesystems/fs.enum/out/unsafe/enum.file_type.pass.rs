extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn ME_0(mut val: ()) -> () {
    return ((val) as ());
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
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "std::is_same<UT, signed char>::value");;
    const _: () = assert!(
        true,
        "E::none == ME(0) &&\n          E::not_found == ME(-1) &&\n          E::regular == ME(1) &&\n          E::directory == ME(2) &&\n          E::symlink == ME(3) &&\n          E::block == ME(4) &&\n          E::character == ME(5) &&\n          E::fifo == ME(6) &&\n          E::socket == ME(7) &&\n          E::unknown == ME(8)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
