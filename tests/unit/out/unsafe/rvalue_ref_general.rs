extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i1: () = 3_();
    let mut i2: () = i1;
    unimplemented!();
    unimplemented!();
    let mut __tmp_0: () = 40_();
    let i3: *mut () = &mut __tmp_0;
    (*i3) += 2_();
    unimplemented!();
    let mut __tmp_1: () = ((2_()) + (3_()));
    let i4: *mut () = &mut __tmp_1;
    unimplemented!();
    let mut __tmp_2: () = 40_();
    let i5: *const () = &mut __tmp_2;
    let i6: *mut () = i3;
    let i7: *mut () = i4;
    unimplemented!();
    unimplemented!();
    let mut i8: () = 3_();
    let i9: *mut () = &mut i8;
    unimplemented!();
    let mut p1: *mut () = (&mut i1 as *mut ());
    let mut p2: *mut () = (i3);
    let mut p3: *mut () = (i6);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
