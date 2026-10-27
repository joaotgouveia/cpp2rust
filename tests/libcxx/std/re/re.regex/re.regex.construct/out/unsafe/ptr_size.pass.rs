extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut p: *const (), mut len: (), mut mc: ()) {
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_0(c"\\(a\\)".as_ptr(), 5_(), 0_()) });
    (unsafe { test_0(c"\\(a[bc]\\)".as_ptr(), 9_(), 0_()) });
    (unsafe { test_0(c"\\(a\\([bc]\\)\\)".as_ptr(), 13_(), 0_()) });
    (unsafe { test_0(c"(a([bc]))".as_ptr(), 9_(), 2_()) });
    (unsafe {
        test_0(
            (&[
                (40 as libc::c_char),
                (0 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (98 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (99 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (100 as libc::c_char),
                (41 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            12_(),
            4_(),
        )
    });
    (unsafe {
        test_0(
            (&[
                (40 as libc::c_char),
                (0 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (98 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (99 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (100 as libc::c_char),
                (41 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            9_(),
            3_(),
        )
    });
    (unsafe {
        test_0(
            (&[
                (40 as libc::c_char),
                (0 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (98 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (99 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (100 as libc::c_char),
                (41 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            3_(),
            1_(),
        )
    });
    (unsafe {
        test_0(
            (&[
                (40 as libc::c_char),
                (0 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (98 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (99 as libc::c_char),
                (41 as libc::c_char),
                (40 as libc::c_char),
                (100 as libc::c_char),
                (41 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            0_(),
            0_(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
