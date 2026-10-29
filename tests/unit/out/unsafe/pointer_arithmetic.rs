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
    let mut x: () = 1_();
    let mut p: *mut () = (&mut x as *mut ());
    (*p) += 1_();
    unimplemented!();
    let mut a: [(); 2] = [1_(), 2_()];
    p = (&mut a[(1_()) as usize] as *mut ());
    (*p) += 1_();
    unimplemented!();
    p.prefix_dec();
    (*p) += 1_();
    unimplemented!();
    p = (&mut x as *mut ());
    (*p) += 1_();
    unimplemented!();
    let mut p2: *mut () = p;
    (*p2) += 1_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
