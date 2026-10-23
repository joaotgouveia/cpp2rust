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
pub struct Y {
    pub i_: (),
}
impl Y {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
pub unsafe fn operator_eq_0(x: *const X, y: *const X) -> () {
    return (((*x).i_) == ((*y).i_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {
    pub i_: (),
}
impl X {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn new_2(y: *const Y) -> Self {
        let mut this = Self { i_: (*y).i_ };
        this
    }
    pub unsafe fn new_3(y: *mut Y) -> Self {
        let mut this = Self {
            i_: (((*y).i_) + (1_())),
        };
        this
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const X, other as *const X) }
    }
}
impl std::cmp::Eq for X {}
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
        let opt: () = unimplemented!();
        let y: Y = Y::new({ 3_() });
        const _: () = assert!(true, "opt.value_or(y) == 2");;
    }
    {
        let opt: () = unimplemented!();
        const _: () = assert!(true, "opt.value_or(Y(3)) == 2");;
    }
    {
        let opt: () = unimplemented!();
        let y: Y = Y::new({ 3_() });
        const _: () = assert!(true, "opt.value_or(y) == 3");;
    }
    {
        let opt: () = unimplemented!();
        const _: () = assert!(true, "opt.value_or(Y(3)) == 4");;
    }
    {
        let opt: () = unimplemented!();
        let y: Y = Y::new({ 3_() });
        unimplemented!();
    }
    {
        let opt: () = unimplemented!();
        unimplemented!();
    }
    {
        let opt: () = unimplemented!();
        let y: Y = Y::new({ 3_() });
        unimplemented!();
    }
    {
        let opt: () = unimplemented!();
        unimplemented!();
    }
    {
        let opt: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
