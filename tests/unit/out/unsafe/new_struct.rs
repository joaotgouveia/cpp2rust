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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Triple {
    pub a: (),
    pub b: (),
    pub p: Pair,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: *mut Pair = (Box::leak(Box::new(Pair { x: 1_(), y: 2_() })) as *mut Pair);
    let mut out: () = (((*p).x) + ((*p).y));
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    unimplemented!();
    let mut t: Triple = Triple {
        a: 1_(),
        b: unimplemented!(),
        p: <Pair>::default(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut q: *mut Triple = (Box::leak(Box::new(Triple {
        a: 2_(),
        b: 3_(),
        p: <Pair>::default(),
    })) as *mut Triple);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let __p = q;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
