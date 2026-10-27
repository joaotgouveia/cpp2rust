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
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_3600__1____std_chrono_duration_long__std_ratio_3600__1___
{}
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
    const _: () = assert!(true, "std::chrono::make12(hours( 0)) == hours(12)");;
    const _: () = assert!(true, "std::chrono::make12(hours(11)) == hours(11)");;
    const _: () = assert!(true, "std::chrono::make12(hours(12)) == hours(12)");;
    const _: () = assert!(true, "std::chrono::make12(hours(23)) == hours(11)");;
    unimplemented!();
    let mut i: () = 1_();
    'loop_: while ((i) < (13_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 13_();
    'loop_: while ((i) < (24_())) {
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
