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
    {
        let mut out: () = unimplemented!();
        let mut sv: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut out: () = unimplemented!();
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"some text".as_ptr(),
            libcc2rs::c_str_len(c"some text".as_ptr()) + 1,
        )
        .to_vec();
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut out: () = unimplemented!();
        let mut sv: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut out: () = unimplemented!();
        let mut s: Vec<i32> = std::slice::from_raw_parts(
            [
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32,
                    101 as i32, 120 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
