extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveAssignUnsafe)]
pub struct S {
    pub a: (),
    pub k: (),
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            a: unimplemented!(),
            k: (*unimplemented!()),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        unimplemented!();
        self.k = (*_a0).k;
        return &mut (*(self as *mut S));
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub unsafe fn getP_0() -> () {
    return unimplemented!();
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
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = 5_();
        (*unimplemented!()) = 6_();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
