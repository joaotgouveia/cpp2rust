extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn new(mut x: ()) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) += x;
        this
    }
}
pub static mut a_1: std::cell::LazyCell<S> =
    std::cell::LazyCell::new(|| unsafe { S::new({ 1_() }) });
pub static mut b_2: std::cell::LazyCell<S> =
    std::cell::LazyCell::new(|| unsafe { S::new({ 10_() }) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
    std::cell::LazyCell::force(&*&raw const a_1);
    std::cell::LazyCell::force(&*&raw const b_2);
}
