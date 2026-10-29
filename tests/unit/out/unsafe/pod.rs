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
pub struct POD {
    pub x1: (),
    pub x2: (),
    pub x3: (),
}
pub unsafe fn PODIncrement_0(pod: *mut POD) {
    (*pod).x1 += 1_();
    (*pod).x2 += 2_();
    (*pod).x3 += 3_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p1: POD = POD {
        x1: 10_(),
        x2: 11_(),
        x3: 12_(),
    };
    let mut p2: POD = POD {
        x1: p1.x1,
        x2: p1.x2,
        x3: p1.x3,
    };
    (unsafe { PODIncrement_0(&mut p2) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
