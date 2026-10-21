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
pub struct S_int_ {
    pub x: (),
}
impl S_int_ {
    pub unsafe fn set(&mut self, mut v: ()) {
        self.x = v;
    }
}
impl Default for S_int_ {
    fn default() -> Self {
        unsafe {
            S_int_ {
                x: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: S_int_ = <S_int_>::default();
    (unsafe { S_int_::set(&mut p, 3_()) });
    unimplemented!();
    return unimplemented!();
}
impl S_int_ {
    pub unsafe fn get(&mut self) -> () {
        return self.x;
    }
}
pub unsafe fn f_0(mut p: *mut S_int_) -> () {
    return (unsafe { S_int_::get(&mut (*p)) });
}
pub unsafe fn __cpp2rust_init_globals() {}
