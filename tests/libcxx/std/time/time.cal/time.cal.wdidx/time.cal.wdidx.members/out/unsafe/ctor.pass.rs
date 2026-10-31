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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let wdi0: () = unimplemented!();
    const _: () = assert!(true, "wdi0.weekday() == weekday{{}}");;
    const _: () = assert!(true, "wdi0.index() == 0");;
    const _: () = assert!(true, "!wdi0.ok()");;
    let wdi1: () = unimplemented!();
    const _: () = assert!(true, "wdi1.weekday() == std::chrono::Sunday");;
    const _: () = assert!(true, "wdi1.index() == 2");;
    const _: () = assert!(true, "wdi1.ok()");;
    let mut i: u32 = 1_u32;
    'loop_: while ((i) <= (5_u32)) {
        let mut wdi: () = unimplemented!();
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (i)));
        assert!(unimplemented!());
        i.prefix_inc();
    }
    let mut i: u32 = 6_u32;
    'loop_: while ((i) <= (20_u32)) {
        let mut wdi: () = unimplemented!();
        assert!(!(unimplemented!()));
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
