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
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            r == std::slice::from_raw_parts(
                c"123-555-1234, 123-555-2345, 123-555-3456".as_ptr(),
                libcc2rs::c_str_len(c"123-555-1234, 123-555-2345, 123-555-3456".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            r == std::slice::from_raw_parts(
                c"123-$555-1234, 123-$555-2345, 123-$555-3456".as_ptr(),
                libcc2rs::c_str_len(c"123-$555-1234, 123-$555-2345, 123-$555-3456".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            r == std::slice::from_raw_parts(
                c"123-555-1234, 123-555-2345, 123-555-3456".as_ptr(),
                libcc2rs::c_str_len(c"123-555-1234, 123-555-2345, 123-555-3456".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            r == std::slice::from_raw_parts(
                c"123-555-1234123-555-2345123-555-3456".as_ptr(),
                libcc2rs::c_str_len(c"123-555-1234123-555-2345123-555-3456".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            r == std::slice::from_raw_parts(
                c"123-555-1234, 555-2345, 555-3456".as_ptr(),
                libcc2rs::c_str_len(c"123-555-1234, 555-2345, 555-3456".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut r: Vec<libc::c_char> = unimplemented!();
        assert!(
            r == std::slice::from_raw_parts(
                c"123-555-1234".as_ptr(),
                libcc2rs::c_str_len(c"123-555-1234".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
