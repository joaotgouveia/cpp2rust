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
    pub v: (),
}
impl S {
    pub unsafe fn operator_eq(&self, mut o: ()) -> () {
        return ((self.v) == (o));
    }
    pub unsafe fn operator_ne(&self, mut o: ()) -> () {
        return ((self.v) != (o));
    }
    pub unsafe fn operator_lt(&self, mut o: ()) -> () {
        return ((self.v) < (o));
    }
    pub unsafe fn operator_gt(&self, mut o: ()) -> () {
        return ((self.v) > (o));
    }
    pub unsafe fn operator_le(&self, mut o: ()) -> () {
        return ((self.v) <= (o));
    }
    pub unsafe fn operator_ge(&self, mut o: *const ()) -> () {
        return ((self.v) >= ((*o) - ('0' as ())));
    }
}
pub unsafe fn operator_eq_0(mut a: (), b: *const S) -> () {
    return ((a) == ((*b).v));
}
pub unsafe fn operator_ne_1(mut a: (), b: *const S) -> () {
    return ((a) != ((*b).v));
}
pub unsafe fn operator_lt_2(mut a: (), b: *const S) -> () {
    return ((a) < ((*b).v));
}
pub unsafe fn operator_gt_3(mut a: (), b: *const S) -> () {
    return ((a) > ((*b).v));
}
pub unsafe fn operator_le_4(mut a: (), b: *const S) -> () {
    return ((a) <= ((*b).v));
}
pub unsafe fn operator_ge_5(mut a: *const (), b: *const S) -> () {
    return (((*a) - ('0' as ())) >= ((*b).v));
}
pub unsafe fn operator_lt_6(a: *mut S, mut b: ()) -> () {
    return ((((*a).v) + (1_())) < (b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 5_() };
    let cs: *const S = &s;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
