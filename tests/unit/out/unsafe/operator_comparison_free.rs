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
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const S, other as *const S) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const S, self as *const S) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const S, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub unsafe fn operator_eq_1(a: *const S, b: *const S) -> () {
    return (((*a).v) == ((*b).v));
}
pub unsafe fn operator_ne_2(a: *const S, b: *const S) -> () {
    return (((*a).v) != ((*b).v));
}
pub unsafe fn operator_lt_0(a: *const S, b: *const S) -> () {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_gt_3(a: *const S, b: *const S) -> () {
    return (((*a).v) > ((*b).v));
}
pub unsafe fn operator_le_4(a: *const S, b: *const S) -> () {
    return (((*a).v) <= ((*b).v));
}
pub unsafe fn operator_ge_5(a: *const S, b: *const S) -> () {
    return (((*a).v) >= ((*b).v));
}
pub unsafe fn operator_lt_6(a: *const S, mut b: ()) -> () {
    return (((*a).v) < (b));
}
pub unsafe fn operator_lt_7(mut a: (), b: *const S) -> () {
    return ((a) < ((*b).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct V {
    pub v: (),
}
impl std::cmp::Ord for V {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_8(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for V {}
pub unsafe fn operator_eq_9(mut a: V, mut b: V) -> () {
    return ((a.v) == (b.v));
}
pub unsafe fn operator_ne_10(mut a: V, mut b: V) -> () {
    return ((a.v) != (b.v));
}
pub unsafe fn operator_lt_8(mut a: V, mut b: V) -> () {
    return ((a.v) < (b.v));
}
pub unsafe fn operator_gt_11(mut a: V, mut b: V) -> () {
    return ((a.v) > (b.v));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: V = V { v: 1_() };
    let mut y: V = V { v: 2_() };
    let mut z: V = V { v: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut a: S = S { v: 1_() };
    let mut b: S = S { v: 2_() };
    let mut c: S = S { v: 1_() };
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
