extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(p: *const Vec<libc::c_char>, mut f: (), mut mc: u32) {
    let mut r: () = unimplemented!();
    assert!(((unimplemented!() as u32) == (f as u32)));
    assert!(((unimplemented!()) == (mc)));
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
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 1_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a[bc]\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a[bc]\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 1_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\([bc]\\)\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\([bc]\\)\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 2_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"(a([bc]))".as_ptr(),
            libcc2rs::c_str_len(c"(a([bc]))".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a[bc]\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a[bc]\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\([bc]\\)\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\([bc]\\)\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"(a([bc]))".as_ptr(),
            libcc2rs::c_str_len(c"(a([bc]))".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 2_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a[bc]\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a[bc]\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\([bc]\\)\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\([bc]\\)\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"(a([bc]))".as_ptr(),
            libcc2rs::c_str_len(c"(a([bc]))".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 2_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a[bc]\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a[bc]\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\([bc]\\)\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\([bc]\\)\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"(a([bc]))".as_ptr(),
            libcc2rs::c_str_len(c"(a([bc]))".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 2_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 1_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a[bc]\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a[bc]\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 1_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\([bc]\\)\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\([bc]\\)\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 2_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"(a([bc]))".as_ptr(),
            libcc2rs::c_str_len(c"(a([bc]))".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a[bc]\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a[bc]\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"\\(a\\([bc]\\)\\)".as_ptr(),
            libcc2rs::c_str_len(c"\\(a\\([bc]\\)\\)".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 0_u32)
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"(a([bc]))".as_ptr(),
            libcc2rs::c_str_len(c"(a([bc]))".as_ptr()) + 1,
        )
        .to_vec();
        test_0(&mut _p, unimplemented!(), 2_u32)
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
