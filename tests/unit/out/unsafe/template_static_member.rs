extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut s_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 55_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_int_ {}
pub static mut s_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 55_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_char_ {}
pub static mut s_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 55_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_long_ {}
pub static mut lo_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut hi_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 1_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_int_ {}
pub static mut lo_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { -2_() });
pub static mut hi_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 2_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_long_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)) = 22_();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut s_1)) = 33_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const s_0);
    std::cell::LazyCell::force(&*&raw const s_1);
    std::cell::LazyCell::force(&*&raw const s_2);
    std::cell::LazyCell::force(&*&raw const lo_3);
    std::cell::LazyCell::force(&*&raw const hi_4);
    std::cell::LazyCell::force(&*&raw const lo_5);
    std::cell::LazyCell::force(&*&raw const hi_6);
}
