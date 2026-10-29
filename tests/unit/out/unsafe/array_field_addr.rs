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
pub struct S {
    pub before: (),
    pub mask: [(); 4],
    pub after: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    (*s).before = 1_();
    unimplemented!();
    (*s).after = 2_();
    (*((&mut (*s).mask as *mut [(); 4]) as *mut ())) = 7_();
    let mut out: [(); 4] = [unimplemented!(); 4];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
