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
    const _: () = assert!(true, "!std::is_convertible<std::string, Myconv>::value");;
    const _: () = assert!(true, "std::is_constructible<Myconv, std::string>::value");;
    {
        let mut myconv: () = unimplemented!();
        let mut bs: Vec<libc::c_char> = unimplemented!();
        assert!(
            bs == std::slice::from_raw_parts(
                c"byte error".as_ptr(),
                libcc2rs::c_str_len(c"byte error".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut myconv: () = unimplemented!();
        let mut bs: Vec<libc::c_char> = unimplemented!();
        assert!(
            bs == std::slice::from_raw_parts(
                c"byte error".as_ptr(),
                libcc2rs::c_str_len(c"byte error".as_ptr()) + 1
            )
            .to_vec()
        );
        let mut ws: Vec<i32> = unimplemented!();
        assert!(
            ws == std::slice::from_raw_parts(
                [
                    119 as i32, 105 as i32, 100 as i32, 101 as i32, 32 as i32, 101 as i32,
                    114 as i32, 114 as i32, 111 as i32, 114 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        119 as i32, 105 as i32, 100 as i32, 101 as i32, 32 as i32, 101 as i32,
                        114 as i32, 114 as i32, 111 as i32, 114 as i32, 0 as i32,
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
