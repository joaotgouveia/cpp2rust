extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_chrono_month__const_std_chrono_month_ {}
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::chrono::January   == std::chrono::month(1)");;
    const _: () = assert!(true, "std::chrono::February  == std::chrono::month(2)");;
    const _: () = assert!(true, "std::chrono::March     == std::chrono::month(3)");;
    const _: () = assert!(true, "std::chrono::April     == std::chrono::month(4)");;
    const _: () = assert!(true, "std::chrono::May       == std::chrono::month(5)");;
    const _: () = assert!(true, "std::chrono::June      == std::chrono::month(6)");;
    const _: () = assert!(true, "std::chrono::July      == std::chrono::month(7)");;
    const _: () = assert!(true, "std::chrono::August    == std::chrono::month(8)");;
    const _: () = assert!(true, "std::chrono::September == std::chrono::month(9)");;
    const _: () = assert!(true, "std::chrono::October   == std::chrono::month(10)");;
    const _: () = assert!(true, "std::chrono::November  == std::chrono::month(11)");;
    const _: () = assert!(true, "std::chrono::December  == std::chrono::month(12)");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
