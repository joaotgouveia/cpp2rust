extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn set_0(r: *mut ()) {
    (*r) = 7_();
}
pub unsafe fn set_through_1(v: *const ()) {
    (unsafe { set_0((v).cast_mut()) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let a: () = 1_();
    let mut p: *const () = (&a as *const ());
    let mut q: *mut () = (((((p) as ()) as ()) as ()) as *mut ());
    unimplemented!();
    let mut v: () = 1_();
    let cr: *const () = &v;
    (unsafe { set_0((cr).cast_mut()) });
    unimplemented!();
    v = unimplemented!();
    (unsafe { set_through_1(&v) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
