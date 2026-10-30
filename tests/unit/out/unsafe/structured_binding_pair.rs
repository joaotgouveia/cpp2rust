extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_std_pair_0(mut x: ()) -> () {
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut __decomp_1: () = (unsafe { make_std_pair_0(5_()) });
    let value: *mut () = unimplemented!();
    let positive: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: () = unimplemented!();
    let mut __decomp_2: () = unimplemented!();
    let a: *mut () = unimplemented!();
    let b: *mut () = unimplemented!();
    (*a) = 10_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_3: *mut () = &mut p;
    let x: *mut () = unimplemented!();
    let y: *mut () = unimplemented!();
    (*x) = 3_();
    (*y) += 4_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
