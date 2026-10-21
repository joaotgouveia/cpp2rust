extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut inner_const_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 1_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
impl C {
    pub unsafe fn get(&mut self) -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_0));
    }
}
pub static mut inner_const_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2_() });
pub type anon_3 = ();
pub const anon_3_kValue: anon_3 = 3;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn f() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2));
    }
}
pub static mut counter_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 10_() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: C = <C>::default();
    unimplemented!();
    unimplemented!();
    let mut s: S = <S>::default();
    let mut p: *mut S = (&mut s as *mut S);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)) = 20_();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)) += 5_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const inner_const_0);
    std::cell::LazyCell::force(&*&raw const inner_const_1);
    std::cell::LazyCell::force(&*&raw const counter_2);
}
