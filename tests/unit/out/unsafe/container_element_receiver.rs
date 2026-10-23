extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub ref_: *mut (),
    pub vec_: (),
}
pub unsafe fn pick_0(a: *mut (), b: *mut (), mut first: ()) -> *mut () {
    return if first { (a) } else { (b) };
}
pub unsafe fn sum_1(v: *const ()) -> () {
    let mut total: () = 0_();
    let mut it: () = unimplemented!();
    'loop_: while unimplemented!() {
        total += (*unimplemented!());
        unimplemented!();
    }
    return ((total) + (*unimplemented!()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    let mut h: Holder = Holder {
        ref_: &mut v,
        vec_: unimplemented!(),
    };
    unimplemented!();
    (*unimplemented!()) = 9_();
    unimplemented!();
    (*unimplemented!()) = 1_();
    unimplemented!();
    let mut p: *mut () = (&mut h.vec_ as *mut ());
    unimplemented!();
    (*unimplemented!()) = 7_();
    unimplemented!();
    let mut m: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: *const () = &v;
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
