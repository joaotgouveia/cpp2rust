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
pub struct Lt {
    pub v: (),
}
impl Lt {
    pub unsafe fn operator_lt(&self, o: *const Lt) -> () {
        return ((self.v) < ((*o).v));
    }
}
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Lt::operator_lt(self, other as *const Lt) {
                std::cmp::Ordering::Less
            } else if Lt::operator_lt(other, self as *const Lt) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Lt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Lt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Lt::operator_lt(self, other as *const Lt))
                && !(Lt::operator_lt(other, self as *const Lt))
        }
    }
}
impl std::cmp::Eq for Lt {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Eq {
    pub v: (),
}
impl Eq {
    pub unsafe fn operator_eq(&self, o: *const Eq) -> () {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Eq::operator_eq(self, other as *const Eq) }
    }
}
impl std::cmp::Eq for Eq {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cmp {
    pub v: (),
}
impl Cmp {
    pub unsafe fn operator_cmp(&self, o: *const Cmp) -> () {
        return std::cmp::Ord::cmp(&(self.v), &((*o).v));
    }
    pub unsafe fn operator_eq(&self, o: *const Cmp) -> () {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Cmp::operator_eq(self, other as *const Cmp) }
    }
}
impl std::cmp::Eq for Cmp {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Free {
    pub v: (),
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const Free, other as *const Free) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const Free, self as *const Free) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Free {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Free, other as *const Free) }
    }
}
impl std::cmp::Eq for Free {}
pub unsafe fn operator_lt_0(a: *const Free, b: *const Free) -> () {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_eq_1(a: *const Free, b: *const Free) -> () {
    return (((*a).v) == ((*b).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Wrapped_int_ {
    pub v: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2_() };
    unimplemented!();
    let mut lts: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut eqs: () = unimplemented!();
    let mut two: Eq = Eq { v: 2_() };
    let mut nine: Eq = Eq { v: 9_() };
    unimplemented!();
    unimplemented!();
    let mut cmps: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut three: Cmp = Cmp { v: 3_() };
    unimplemented!();
    let mut frees: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ftwo: Free = Free { v: 2_() };
    unimplemented!();
    let mut m: () = unimplemented!();
    (*unimplemented!()) = 20_();
    (*unimplemented!()) = 10_();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
