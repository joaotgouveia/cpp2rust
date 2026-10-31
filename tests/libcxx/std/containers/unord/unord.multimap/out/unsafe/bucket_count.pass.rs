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
        let c: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 8] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c: () = unimplemented!();
        assert!(((unimplemented!()) >= (8_())));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
