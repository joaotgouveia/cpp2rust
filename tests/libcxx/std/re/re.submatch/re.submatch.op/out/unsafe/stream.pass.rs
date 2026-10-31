extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(s: *const Vec<libc::c_char>) {
    let mut sm: () = unimplemented!();
    (sm as (
        UnsafeRandomAccessIterator<libc::c_char>,
        UnsafeRandomAccessIterator<libc::c_char>,
    ))
        .0 = unimplemented!();
    (sm as (
        UnsafeRandomAccessIterator<libc::c_char>,
        UnsafeRandomAccessIterator<libc::c_char>,
    ))
        .1 = unimplemented!();
    sm.matched = true;
    let mut os: () = unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_1(s: *const Vec<i32>) {
    let mut sm: () = unimplemented!();
    (sm as (
        UnsafeRandomAccessIterator<i32>,
        UnsafeRandomAccessIterator<i32>,
    ))
        .0 = unimplemented!();
    (sm as (
        UnsafeRandomAccessIterator<i32>,
        UnsafeRandomAccessIterator<i32>,
    ))
        .1 = unimplemented!();
    sm.matched = true;
    let mut os: () = unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe {
        let mut _s: Vec<libc::c_char> =
            std::slice::from_raw_parts(c"123".as_ptr(), libcc2rs::c_str_len(c"123".as_ptr()) + 1)
                .to_vec();
        test_0(&mut _s)
    });
    (unsafe {
        let mut _s: Vec<i32> = std::slice::from_raw_parts(
            [49 as i32, 50 as i32, 51 as i32, 0 as i32].as_ptr(),
            libcc2rs::c_str_len([49 as i32, 50 as i32, 51 as i32, 0 as i32].as_ptr()) + 1,
        )
        .to_vec();
        test_1(&mut _s)
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
