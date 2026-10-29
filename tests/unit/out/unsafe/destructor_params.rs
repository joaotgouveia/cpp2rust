extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut destroyed_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut last_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {
    pub v: (),
}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).postfix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) = self.v;
    }
}
pub unsafe fn by_value_2(mut s: S) -> () {
    unimplemented!();
    return s.v;
}
pub unsafe fn with_default_3(mut s: Option<S>) -> () {
    let mut s: S = s.unwrap_or_else(|| unsafe { S { v: 5_() } });
    return s.v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub v: (),
}
impl Holder {
    pub unsafe fn new(mut s: S) -> Self {
        let mut this = Self { v: s.v };
        this
    }
    pub unsafe fn add(&self, mut s: S) -> () {
        return ((self.v) + (s.v));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    {
        let mut s: S = S { v: 1_() };
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    unimplemented!();
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    {
        let mut h: Holder = Holder::new({ S { v: 3_() } });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destroyed_0);
    std::cell::LazyCell::force(&*&raw const last_1);
}
