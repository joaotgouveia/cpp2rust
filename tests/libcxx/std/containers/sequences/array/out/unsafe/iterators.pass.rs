extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoDefault {}
impl NoDefault {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn check_noexcept_0(c: *mut ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const () = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_1(c: *mut ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const () = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_2(c: *mut ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const () = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_3(c: *mut ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const () = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_4(c: *mut ()) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const () = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn tests_5() -> () {
    {
        let mut array: () = unimplemented!();
        (unsafe { check_noexcept_0(&mut array) });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        (unsafe { check_noexcept_1(&mut array) });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        (unsafe { check_noexcept_1(&mut array) });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        (unsafe { check_noexcept_2(&mut array) });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*i) = 99_();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        (unsafe { check_noexcept_3(&mut array) });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*i) = 99_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        (unsafe { check_noexcept_4(&mut array) });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*i) = 5.5_f64;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut array: () = unimplemented!();
        let mut ib: () = unimplemented!();
        let mut ie: () = unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ii1: () = unimplemented!();
            let mut ii2: () = unimplemented!();
            let mut ii4: () = ii1;
            let mut cii: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut c: () = unimplemented!();
            (unsafe { check_noexcept_0(&mut c) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut r1: () = std::cmp::Ord::cmp(&(ii1), &(ii2));
            unimplemented!();
            let mut r2: () = std::cmp::Ord::cmp(&(cii), &((ii2).cast_const()));
            unimplemented!();
        }
        {
            let mut ii1: () = unimplemented!();
            let mut ii2: () = unimplemented!();
            let mut ii4: () = ii1;
            let mut cii: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut c: () = unimplemented!();
            (unsafe { check_noexcept_1(&mut c) });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut r1: () = std::cmp::Ord::cmp(&(ii1), &(ii2));
            unimplemented!();
            let mut r2: () = std::cmp::Ord::cmp(&(cii), &((ii2).cast_const()));
            unimplemented!();
        }
    }
    return true;
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
    (unsafe { tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
