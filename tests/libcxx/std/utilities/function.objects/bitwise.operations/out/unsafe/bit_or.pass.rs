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
    let f: () = unimplemented!();
    assert!(((unimplemented!()) == (60053)));
    assert!(((unimplemented!()) == (64215)));
    assert!(((unimplemented!()) == (64215)));
    assert!(((unimplemented!()) == (22739)));
    assert!(((unimplemented!()) == (65535)));
    let f2: () = unimplemented!();
    assert!(((unimplemented!()) == (60053)));
    assert!(((unimplemented!()) == (60053_i64)));
    assert!(((unimplemented!()) == (60053_i64)));
    assert!(((unimplemented!()) == (64215)));
    assert!(((unimplemented!()) == (64215_i64)));
    assert!(((unimplemented!()) == (64215_i64)));
    assert!(((unimplemented!()) == (64215)));
    assert!(((unimplemented!()) == (64215_i64)));
    assert!(((unimplemented!()) == (64215_i64)));
    assert!(((unimplemented!()) == (22739)));
    assert!(((unimplemented!()) == (22739_i64)));
    assert!(((unimplemented!()) == (22739_i64)));
    assert!(((unimplemented!()) == (65535)));
    assert!(((unimplemented!()) == (65535_i64)));
    assert!(((unimplemented!()) == (65535_i64)));
    let foo: i32 = unimplemented!();
    const _: () = assert!(true, "foo == 0xFAD7");;
    let bar: i32 = (unimplemented!() as i32);
    const _: () = assert!(true, "bar == 0xFAD7");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
