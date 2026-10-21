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
    pub unsafe fn operator_eq(&self, o: *const S) -> () {
        return ((self.v) == ((*o).v));
    }
    pub unsafe fn operator_ne(&self, o: *const S) -> () {
        return ((self.v) != ((*o).v));
    }
    pub unsafe fn operator_lt_3(&self, o: *const S) -> () {
        return ((self.v) < ((*o).v));
    }
    pub unsafe fn operator_gt(&self, o: *const S) -> () {
        return ((self.v) > ((*o).v));
    }
    pub unsafe fn operator_le(&self, o: *const S) -> () {
        return ((self.v) <= ((*o).v));
    }
    pub unsafe fn operator_ge(&self, o: *const S) -> () {
        return ((self.v) >= ((*o).v));
    }
    pub unsafe fn operator_lt_7(&self, mut o: ()) -> () {
        return ((self.v) < (o));
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if S::operator_lt_3(self, other as *const S) {
                std::cmp::Ordering::Less
            } else if S::operator_lt_3(other, self as *const S) {
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
        unsafe { S::operator_eq(self, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
