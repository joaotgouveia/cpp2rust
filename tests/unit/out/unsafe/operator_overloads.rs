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
    pub unsafe fn operator_eq_1(&self, mut o: ()) -> () {
        return if ((self.v) == (o)) { 1_() } else { 0_() };
    }
    pub unsafe fn operator_eq_2(&self, mut o: ()) -> () {
        return if ((self.v) == (o)) { 2_() } else { 0_() };
    }
    pub unsafe fn operator_eq_3(&self, mut o: ()) -> () {
        return if ((self.v) == (o)) { 3_() } else { 0_() };
    }
    pub unsafe fn operator_add(&self, o: *const S) -> () {
        return ((self.v) + ((*o).v));
    }
    pub unsafe fn operator_sub(&self, mut o: S) -> () {
        return ((self.v) - (o.v));
    }
    pub unsafe fn operator_mul_6(&self, o: *const S) -> () {
        return ((self.v) * ((*o).v));
    }
    pub unsafe fn operator_mul_7(&self, mut o: ()) -> () {
        return (((self.v) * (o)) + (1_()));
    }
}
pub unsafe fn operator_div_0(a: *const S, b: *const S) -> () {
    return (((*a).v) / ((*b).v));
}
pub unsafe fn operator_div_1(mut a: S, mut b: ()) -> () {
    return (((a.v) / (b)) + (1_()));
}
pub unsafe fn operator_rem_2(mut a: S, mut b: S) -> () {
    return ((a.v) % (b.v));
}
pub unsafe fn operator_rem_3(a: *const S, mut b: ()) -> () {
    return ((((*a).v) % (b)) + (1_()));
}
pub unsafe fn operator_eq_4(mut a: (), mut b: S) -> () {
    return if ((a) == (b.v)) { 4_() } else { 0_() };
}
pub unsafe fn operator_eq_5(mut a: (), b: *const S) -> () {
    return if ((a) == ((*b).v)) { 5_() } else { 0_() };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 6_() };
    let mut t: S = S { v: 4_() };
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
