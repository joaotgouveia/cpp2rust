extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn more_refs_0(mut x1: (), mut x2: (), r1: *mut (), r2: *const ()) {
    let rx1: *const () = &x1;
    let rx2: *mut () = &mut x2;
    let mut pr1: *const () = (r1).cast_const();
    let mut pr2: *const () = (r2);
    let rpr1: *const () = &(*pr1);
    let rpr2: *const () = &(*pr2);
    let r: *const () = r1;
    (*rx2) += ((((((((1_()) + (*rx1)) + (*rx2)) + (*pr1)) + (*pr2)) + (*rpr1)) + (*rpr2)) + (*r));
    (*r1) = (*rx2);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Val {
    pub x: (),
}
pub unsafe fn sum_1(mut a: Val, mut b: Val) -> () {
    return ((a.x) + (b.x));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = 1_();
    let x2: () = 2_();
    (unsafe { more_refs_0(3_(), 4_(), &mut x1, &x2) });
    unimplemented!();
    let mut v: Val = Val { x: 5_() };
    let mut acc: () = (unsafe {
        let _a: Val = v;
        let _b: Val = v;
        sum_1(_a, _b)
    });
    acc += (unsafe {
        let _a: Val = v;
        let _b: Val = v;
        sum_1(_a, _b)
    });
    acc += (unsafe {
        let _a: Val = v;
        let _b: Val = v;
        sum_1(_a, _b)
    });
    acc += (unsafe {
        let _a: Val = v;
        let _b: Val = v;
        sum_1(_a, _b)
    });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
