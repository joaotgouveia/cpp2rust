extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct gnu_obj {
    i: (),
}
impl gnu_obj {
    pub unsafe fn new(mut arg: Option<()>) -> Self {
        let mut arg: () = arg.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { i: arg };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const gnu_obj) -> () {
        return ((self.i) == ((*rhs).i));
    }
    pub unsafe fn operator_ne(&self, rhs: *const gnu_obj) -> () {
        return ((self.i) != ((*rhs).i));
    }
    pub unsafe fn operator_lt(&self, rhs: *const gnu_obj) -> () {
        return ((self.i) < ((*rhs).i));
    }
}
impl std::cmp::Ord for gnu_obj {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if gnu_obj::operator_lt(self, other as *const gnu_obj) {
                std::cmp::Ordering::Less
            } else if gnu_obj::operator_lt(other, self as *const gnu_obj) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for gnu_obj {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for gnu_obj {
    fn eq(&self, other: &Self) -> bool {
        unsafe { gnu_obj::operator_eq(self, other as *const gnu_obj) }
    }
}
impl std::cmp::Eq for gnu_obj {}
impl Default for gnu_obj {
    fn default() -> Self {
        unsafe { gnu_obj::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct gnu_t_long_ {
    pub b: (),
}
impl gnu_t_long_ {
    pub unsafe fn operator_ne(&self, rhs: *const gnu_t_long_) -> () {
        return ((self.b) != ((*rhs).b));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
