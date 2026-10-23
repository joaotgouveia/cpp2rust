extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct S {
    pub x: (),
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self { x: 1_() };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut cnt_dtor_0)).prefix_inc();
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub static mut cnt_dtor_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });

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
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const cnt_dtor_0);
}
