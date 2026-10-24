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
pub struct no_allocate_at_least_int_ {
    pub t: (),
}
impl no_allocate_at_least_int_ {
    pub unsafe fn allocate(&mut self, mut _a0: ()) -> *mut () {
        return (&mut self.t as *mut ());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_allocate_at_least_int_ {
    pub t1: (),
    pub t2: (),
}
impl has_allocate_at_least_int_ {
    pub unsafe fn allocate_at_least(&mut self, mut _a0: ()) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut alloc: no_allocate_at_least_int_ = <no_allocate_at_least_int_>::default();
        let mut ret: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut alloc: has_allocate_at_least_int_ = <has_allocate_at_least_int_>::default();
        let mut ret: () = unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
