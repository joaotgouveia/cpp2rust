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
    pub n: (),
    pub name: [(); 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E {
    pub id: (),
    pub w: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct T {
    pub n: (),
    pub cap: (),
    pub a: [E; 1],
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
    unimplemented!();
    (*s).n = 5_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut t: *mut T = (unimplemented!() as *mut T);
    unimplemented!();
    (*t).n = 2_();
    (*t).cap = 2_();
    (*(*t).a.as_mut_ptr().add((0_()) as usize)).id = 10_();
    (*(*t).a.as_mut_ptr().add((1_()) as usize)).w = 20_();
    (*t).n = 3_();
    unimplemented!();
    let mut tail: *mut E = ((&mut (*t.offset((1_()) as isize)) as *mut T) as *mut T as *mut E);
    unimplemented!();
    (*tail.offset((0_()) as isize)).id = 30_();
    (*t).cap = 4_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
