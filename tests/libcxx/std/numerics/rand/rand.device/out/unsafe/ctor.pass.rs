extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: i32) -> bool {
    return true;
}
pub unsafe fn test_convertible_1() -> bool {
    return (unsafe { test_convertible_imp_0(0) });
}
pub unsafe fn check_random_device_valid_2(token: *const Vec<libc::c_char>) {
    let mut r: () = unimplemented!();
}
pub unsafe fn check_random_device_invalid_3(token: *const Vec<libc::c_char>) {
    (&(*token));
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
    {
        let mut r: () = unimplemented!();
        &(r);
    }
    {
        (unsafe {
            let mut _token: Vec<libc::c_char> = std::slice::from_raw_parts(
                c"/dev/urandom".as_ptr(),
                libcc2rs::c_str_len(c"/dev/urandom".as_ptr()) + 1,
            )
            .to_vec();
            check_random_device_valid_2(&mut _token)
        });
        (unsafe {
            let mut _token: Vec<libc::c_char> = std::slice::from_raw_parts(
                c"/dev/random".as_ptr(),
                libcc2rs::c_str_len(c"/dev/random".as_ptr()) + 1,
            )
            .to_vec();
            check_random_device_invalid_3(&mut _token)
        });
        (unsafe {
            let mut _token: Vec<libc::c_char> = std::slice::from_raw_parts(
                c"/dev/null".as_ptr(),
                libcc2rs::c_str_len(c"/dev/null".as_ptr()) + 1,
            )
            .to_vec();
            check_random_device_invalid_3(&mut _token)
        });
        (unsafe {
            let mut _token: Vec<libc::c_char> = std::slice::from_raw_parts(
                c"/dev/nonexistent".as_ptr(),
                libcc2rs::c_str_len(c"/dev/nonexistent".as_ptr()) + 1,
            )
            .to_vec();
            check_random_device_invalid_3(&mut _token)
        });
        (unsafe {
            let mut _token: Vec<libc::c_char> = std::slice::from_raw_parts(
                c"wrong file".as_ptr(),
                libcc2rs::c_str_len(c"wrong file".as_ptr()) + 1,
            )
            .to_vec();
            check_random_device_invalid_3(&mut _token)
        });
    }
    {
        let mut ec: i32 = 0_i32;
        ec = libc::close(0);
        assert!(!(ec != 0));
        ec = libc::close(1);
        assert!(!(ec != 0));
        ec = libc::close(2);
        assert!(!(ec != 0));
        let mut r: () = unimplemented!();
    }
    const _: () = assert!(true, "test_convertible<std::random_device>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
