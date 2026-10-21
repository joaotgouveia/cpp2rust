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
pub struct Static {}
impl Static {
    pub unsafe fn operator_call(mut a: (), mut b: ()) -> () {
        return ((a) * (b));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: (),
}
impl S {
    pub unsafe fn operator_call_1(&self) -> () {
        return self.v;
    }
    pub unsafe fn operator_call_2(&self, mut a: ()) -> () {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_call_3(&self, mut a: (), mut b: ()) -> () {
        return (((self.v) + (a)) + (b));
    }
    pub unsafe fn operator_comma(&self, o: *const S) -> S {
        return S {
            v: (((self.v) * (10_())) + ((*o).v)),
        };
    }
    pub unsafe fn to__5(&self) -> () {
        return self.v;
    }
    pub unsafe fn to__6(&self) -> () {
        return ((self.v) != (0_()));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 3_() };
    let mut t: S = S { v: 4_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = (unsafe { S::to__5(&s) });
    unimplemented!();
    unimplemented!();
    if (unsafe { S::to__6(&s) }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
    let mut z: S = S { v: 0_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut st: Static = <Static>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
