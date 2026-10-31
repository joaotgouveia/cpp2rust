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
    let mut storage: Vec<libc::c_char> = vec![libc::c_char::default()];
    {
        let mut bytestream: () = unimplemented!();
        let mut mybuf: () = unimplemented!();
        let mut mystr: () = unimplemented!();
        {
            let __r: &mut ::std::fs::File = &mut (*unimplemented!());
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        };
        storage = unimplemented!();
    }
    {
        let mut bytestream: () = unimplemented!();
        let mut mybuf: () = unimplemented!();
        let mut mystr: () = unimplemented!();
        let mut ws: Vec<i32> = vec![i32::default()];
        unimplemented!();
        assert!(
            ws == std::slice::from_raw_parts(
                [
                    72 as i32, 101 as i32, 108 as i32, 108 as i32, 111 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        72 as i32, 101 as i32, 108 as i32, 108 as i32, 111 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
