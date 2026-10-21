extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {
    pub v: (),
}
impl S {
    pub unsafe fn const_method(&self) -> () {
        return ((self.v) * (2_()));
    }
    pub unsafe fn mut_method(&mut self) {
        self.v += 1_();
    }
    pub unsafe fn new(mut init: ()) -> Self {
        let mut this = Self { v: init };
        (unsafe { S::mut_method(&mut this) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { S::mut_method(self) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(self) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Point {
    pub x: (),
    pub y: (),
}
impl Point {
    pub unsafe fn new_1(mut x: (), mut y: ()) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Point::new_1({ v }, { ((v) + (1_())) });
        this.y *= 10_();
        this
    }
    pub unsafe fn new_3() -> Self {
        let mut this = Point::new_2({ 4_() });
        this.x += 100_();
        this
    }
}
impl Default for Point {
    fn default() -> Self {
        unsafe { Point::new_3() }
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
        let mut s: S = S::new({ 3_() });
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    let mut p: Point = Point::new_3();
    unimplemented!();
    unimplemented!();
    let mut q: Point = Point::new_2({ 7_() });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
}
