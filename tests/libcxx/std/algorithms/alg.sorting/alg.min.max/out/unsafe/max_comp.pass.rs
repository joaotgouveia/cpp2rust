extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(a: *const (), b: *const (), mut c: (), x: *const ()) {
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
    {
        let mut x: () = 0_();
        let mut y: () = 0_();
        (unsafe {
            let _a: *const () = &x;
            let _x: *const () = &x;
            test_0(_a, &y, unimplemented!(), _x)
        });
        (unsafe {
            let _a: *const () = &y;
            let _x: *const () = &y;
            test_0(_a, &x, unimplemented!(), _x)
        });
    }
    {
        let mut x: () = 0_();
        let mut y: () = 1_();
        (unsafe {
            let _a: *const () = &x;
            let _x: *const () = &x;
            test_0(_a, &y, unimplemented!(), _x)
        });
        (unsafe {
            let _b: *const () = &x;
            let _x: *const () = &x;
            test_0(&y, _b, unimplemented!(), _x)
        });
    }
    {
        let mut x: () = 1_();
        let mut y: () = 0_();
        (unsafe {
            let _b: *const () = &y;
            let _x: *const () = &y;
            test_0(&x, _b, unimplemented!(), _x)
        });
        (unsafe {
            let _a: *const () = &y;
            let _x: *const () = &y;
            test_0(_a, &x, unimplemented!(), _x)
        });
    }
    {
        let x: () = 1_();
        let y: () = 0_();
        const _: () = assert!(true, "std::max(x, y, std::greater<int>()) == y");;
        const _: () = assert!(true, "std::max(y, x, std::greater<int>()) == y");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
