extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(a: *const i32, b: *const i32, mut c: (), x: *const i32) {
    assert!(((unimplemented!()) == (x)));
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
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        (unsafe {
            let _a: *const i32 = &x;
            let _x: *const i32 = &x;
            test_0(_a, &y, unimplemented!(), _x)
        });
        (unsafe {
            let _a: *const i32 = &y;
            let _x: *const i32 = &y;
            test_0(_a, &x, unimplemented!(), _x)
        });
    }
    {
        let mut x: i32 = 0;
        let mut y: i32 = 1;
        (unsafe {
            let _b: *const i32 = &y;
            let _x: *const i32 = &y;
            test_0(&x, _b, unimplemented!(), _x)
        });
        (unsafe {
            let _a: *const i32 = &y;
            let _x: *const i32 = &y;
            test_0(_a, &x, unimplemented!(), _x)
        });
    }
    {
        let mut x: i32 = 1;
        let mut y: i32 = 0;
        (unsafe {
            let _a: *const i32 = &x;
            let _x: *const i32 = &x;
            test_0(_a, &y, unimplemented!(), _x)
        });
        (unsafe {
            let _b: *const i32 = &x;
            let _x: *const i32 = &x;
            test_0(&y, _b, unimplemented!(), _x)
        });
    }
    {
        let x: i32 = 1;
        let y: i32 = 0;
        const _: () = assert!(true, "std::min(x, y, std::greater<int>()) == x");;
        const _: () = assert!(true, "std::min(y, x, std::greater<int>()) == x");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
