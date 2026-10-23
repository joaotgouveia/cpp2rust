extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(a: *const (), b: *const (), x: *const (), y: *const ()) {
    let mut p: () = unimplemented!();
    unimplemented!();
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
            let _b: *const () = &y;
            let _x: *const () = &x;
            let _y: *const () = &y;
            test_0(_a, _b, _x, _y)
        });
        (unsafe {
            let _a: *const () = &y;
            let _b: *const () = &x;
            let _x: *const () = &y;
            let _y: *const () = &x;
            test_0(_a, _b, _x, _y)
        });
    }
    {
        let mut x: () = 0_();
        let mut y: () = 1_();
        (unsafe {
            let _a: *const () = &x;
            let _b: *const () = &y;
            let _x: *const () = &x;
            let _y: *const () = &y;
            test_0(_a, _b, _x, _y)
        });
        (unsafe {
            let _a: *const () = &y;
            let _b: *const () = &x;
            let _x: *const () = &x;
            let _y: *const () = &y;
            test_0(_a, _b, _x, _y)
        });
    }
    {
        let mut x: () = 1_();
        let mut y: () = 0_();
        (unsafe {
            let _a: *const () = &x;
            let _b: *const () = &y;
            let _x: *const () = &y;
            let _y: *const () = &x;
            test_0(_a, _b, _x, _y)
        });
        (unsafe {
            let _a: *const () = &y;
            let _b: *const () = &x;
            let _x: *const () = &y;
            let _y: *const () = &x;
            test_0(_a, _b, _x, _y)
        });
    }
    {
        let p1: () = unimplemented!();
        const _: () = assert!(true, "p1.first  == y");;
        const _: () = assert!(true, "p1.second == x");;
        let p2: () = unimplemented!();
        const _: () = assert!(true, "p2.first  == y");;
        const _: () = assert!(true, "p2.second == x");;
    }
    return 0_();
}
static mut x_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 1_() });
static mut y_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn __cpp2rust_init_globals() {}
