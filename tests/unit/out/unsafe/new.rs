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
pub struct Pair {
    pub x: (),
    pub y: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: *mut () = (Box::leak(Box::new(5_())) as *mut ());
    let mut out: () = (*x);
    {
        let __p = x;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    unimplemented!();
    let mut y: *mut () = (Box::leak(Box::new(unimplemented!())) as *mut ());
    (*y) = 9_();
    unimplemented!();
    {
        let __p = y;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut p: *mut Pair = (Box::leak(Box::new(<Pair>::default())) as *mut Pair);
    (*p).x = 1_();
    (*p).y = 2_();
    unimplemented!();
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut nullpointer: *mut () = std::ptr::null_mut();
    {
        let __p = nullpointer;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
