extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_eprintf_0(
    mut fmt: *const libc::c_char,
    args_1: *const *const libc::c_char,
    args_2: *const *const libc::c_char,
    args_3: *const *const libc::c_char,
    args_4: *const i32,
    args_5: *const *const libc::c_char,
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            (*args_4),
            (*args_5),
        )
    });
}
pub unsafe fn test_log_1(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    mut message: *const libc::c_char,
) {
    let mut msg: *const libc::c_char = if !(condition).is_null() {
        c"Assertion failure: ".as_ptr()
    } else {
        c"Unconditional failure:".as_ptr()
    };
    (unsafe {
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
const _: () = assert!(true, "std::is_signed<std::messages_base::catalog>::value");
const _: () = assert!(true, "std::is_integral<std::messages_base::catalog>::value");
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
    let mut mb: () = unimplemented!();
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
