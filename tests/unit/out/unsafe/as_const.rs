extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Overload = ();
pub const Overload_kMutableOverload: Overload = 1;
pub const Overload_kConstOverload: Overload = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: (),
}
impl S {
    pub unsafe fn f_1(&mut self) -> Overload {
        return Overload_kMutableOverload;
    }
    pub unsafe fn f_2(&self) -> Overload {
        return Overload_kConstOverload;
    }
    pub unsafe fn value_ref_3(&mut self) -> *mut () {
        return &mut self.v;
    }
    pub unsafe fn value_ref_4(&self) -> *const () {
        return &self.v;
    }
}
pub unsafe fn g_0(_a0: *mut S) -> Overload {
    return Overload_kMutableOverload;
}
pub unsafe fn g_1(_a0: *const S) -> Overload {
    return Overload_kConstOverload;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 7_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*(unsafe { S::value_ref_3(&mut s) })) = 9_();
    unimplemented!();
    unimplemented!();
    let cs: *const S = &(*unimplemented!());
    unimplemented!();
    unimplemented!();
    let mut p: *mut S = unimplemented!();
    (*p).v = 11_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
