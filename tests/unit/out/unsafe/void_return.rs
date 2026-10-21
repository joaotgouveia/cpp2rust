extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f1_0(mut first: (), mut last: ()) {
    unimplemented!();
    return;
}
pub unsafe fn f1_1(mut first: *mut (), mut last: *mut ()) {
    unimplemented!();
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    (unsafe {
        let _first: () = unimplemented!();
        let _last: () = unimplemented!();
        f1_0(_first, _last)
    });
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    (unsafe {
        let _first: *mut () = a.as_mut_ptr();
        let _last: *mut () = a.as_mut_ptr().offset((2_()) as isize);
        f1_1(_first, _last)
    });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
