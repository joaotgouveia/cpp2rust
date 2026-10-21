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
pub struct In {
    pub x: (),
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub in_: In,
    pub total: (),
    pub n: (),
    pub arr: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Node {
    pub x: (),
    pub self_: *mut Node,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    let mut q: *mut S = p;
    (*p).in_.x = 1_();
    (*p).in_.y = 2_();
    (*p).total = (((*q).in_.x) + ((*q).in_.y));
    unimplemented!();
    let mut ip: *mut In = (&mut (*p).in_ as *mut In);
    (*ip).x = (((*p).total) + (1_()));
    unimplemented!();
    (*p).arr[((*p).n) as usize] = (*p).total;
    (*p).n += 1_();
    (*p).arr[((*p).n) as usize] = (*q).in_.x;
    unimplemented!();
    unimplemented!();
    let mut s: Node = <Node>::default();
    s.x = 1_();
    s.self_ = (&mut s as *mut Node);
    (*s.self_).x = ((s.x) + (1_()));
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
