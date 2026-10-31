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
    let mut what_arg: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"test message".as_ptr(),
        libcc2rs::c_str_len(c"test message".as_ptr()) + 1,
    )
    .to_vec();
    let mut se: () = unimplemented!();
    assert!(unimplemented!());
    let mut what_message: Vec<libc::c_char> =
        std::slice::from_raw_parts(unimplemented!(), libcc2rs::c_str_len(unimplemented!()) + 1)
            .to_vec();
    assert!(((unimplemented!()) != (usize::MAX)));
    assert!(
        (({
            let __hay = &what_message[..what_message.len() - 1];
            let __needle = ::std::slice::from_raw_parts(
                c"Not a directory".as_ptr(),
                libcc2rs::c_str_len(c"Not a directory".as_ptr()),
            );
            (0_usize..=__hay.len().saturating_sub(__needle.len()))
                .find(|&__i| __hay[__i..].starts_with(__needle))
                .unwrap_or(usize::MAX)
        }) != (usize::MAX))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
