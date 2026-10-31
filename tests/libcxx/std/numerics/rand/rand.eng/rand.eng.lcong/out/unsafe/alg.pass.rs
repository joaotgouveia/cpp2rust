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
    let mut e1: () = unimplemented!();
    assert!(((unimplemented!()) == (25214903918_u64)));
    assert!(((unimplemented!()) == (205774354444503_u64)));
    assert!(((unimplemented!()) == (158051849450892_u64)));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    let mut e2: () = unimplemented!();
    assert!(((unimplemented!()) == ((1_u64) << (32))));
    assert!(((unimplemented!()) == ((((1_u64) << (63)) as u64).wrapping_sub((1_u64 as u64)))));
    assert!(
        ((unimplemented!()) == ((((1_u64) << (63)) as u64).wrapping_sub((8589934591_u64 as u64))))
    );
    assert!(((unimplemented!()) < ((((1_u64) << (63)) as u64).wrapping_add(1_u64))));
    assert!(((unimplemented!()) < ((((1_u64) << (63)) as u64).wrapping_add(1_u64))));
    assert!(((unimplemented!()) < ((((1_u64) << (63)) as u64).wrapping_add(1_u64))));
    assert!(((unimplemented!()) < ((((1_u64) << (63)) as u64).wrapping_add(1_u64))));
    assert!(((unimplemented!()) < ((((1_u64) << (63)) as u64).wrapping_add(1_u64))));
    let mut e3: () = unimplemented!();
    assert!(((unimplemented!()) == (402727752_u64)));
    assert!(((unimplemented!()) == (162159612030764687_u64)));
    assert!(((unimplemented!()) == (108176466184989142_u64)));
    assert!(((unimplemented!()) < ((3_u64) << (56))));
    assert!(((unimplemented!()) < ((3_u64) << (56))));
    assert!(((unimplemented!()) < ((3_u64) << (56))));
    assert!(((unimplemented!()) < ((3_u64) << (56))));
    assert!(((unimplemented!()) < ((3_u64) << (56))));
    let mut e4: () = unimplemented!();
    assert!(((unimplemented!()) == (65545_u32)));
    assert!(((unimplemented!()) == (1179731_u32)));
    assert!(((unimplemented!()) == (16057103_u32)));
    assert!(((unimplemented!()) < (2147483647_u32)));
    assert!(((unimplemented!()) < (2147483647_u32)));
    assert!(((unimplemented!()) < (2147483647_u32)));
    assert!(((unimplemented!()) < (2147483647_u32)));
    assert!(((unimplemented!()) < (2147483647_u32)));
    let mut e5: () = unimplemented!();
    assert!(((unimplemented!()) == (4294967297_u64)));
    assert!(((unimplemented!()) == (8589934601_u64)));
    assert!(((unimplemented!()) == (47244640281_u64)));
    assert!(((unimplemented!()) < ((((1_u64) << (61)) as u64).wrapping_sub((1_u64 as u64)))));
    assert!(((unimplemented!()) < ((((1_u64) << (61)) as u64).wrapping_sub((1_u64 as u64)))));
    assert!(((unimplemented!()) < ((((1_u64) << (61)) as u64).wrapping_sub((1_u64 as u64)))));
    assert!(((unimplemented!()) < ((((1_u64) << (61)) as u64).wrapping_sub((1_u64 as u64)))));
    assert!(((unimplemented!()) < ((((1_u64) << (61)) as u64).wrapping_sub((1_u64 as u64)))));
    let mut e6: () = unimplemented!();
    assert!(((unimplemented!()) == (2_u64)));
    assert!(((unimplemented!()) == (3_u64)));
    assert!(((unimplemented!()) == (4_u64)));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    assert!(((unimplemented!()) < ((1_u64) << (48))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
