extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a: (), mut b: Option<()>) -> () {
    let mut b: () = b.unwrap_or_else(|| unsafe { 10_() });
    return ((a) + (b));
}
pub unsafe fn baz_1(mut a: *mut (), mut b: Option<*mut ()>) -> () {
    let mut b: *mut () = b.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    return ((a) == (b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Bar {
    pub v: (),
}
impl Bar {
    pub unsafe fn new(mut v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { v: v };
        this
    }
}
impl Default for Bar {
    fn default() -> Self {
        unsafe { Bar::new(None) }
    }
}
pub static mut counter_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn next_3() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)).prefix_inc();
}
pub unsafe fn lazy_4(mut x: Option<()>) -> () {
    let mut x: () = x.unwrap_or_else(|| unsafe { (unsafe { next_3() }) });
    return x;
}
pub unsafe fn by_ref_5(b: Option<*const Bar>) -> () {
    let mut __b_default: Option<Bar> = None;
    let mut b: *const Bar =
        b.unwrap_or_else(|| unsafe { __b_default.insert(Bar::new({ Some(7_()) })) as *const Bar });
    return (*b).v;
}
pub static mut global_bar_6: std::cell::LazyCell<Bar> =
    std::cell::LazyCell::new(|| unsafe { Bar::new({ Some(9_()) }) });
pub unsafe fn by_global_ref_7(b: Option<*const Bar>) -> () {
    let mut b: *const Bar = b.unwrap_or_else(|| unsafe {
        &(*std::cell::LazyCell::force_mut(&mut *&raw mut global_bar_6))
    });
    return (*b).v;
}
pub unsafe fn shifted_8(mut x: (), mut y: Option<()>) -> () {
    let mut y: () = y.unwrap_or_else(|| unsafe { -1_() });
    return ((x) + (y));
}
pub unsafe fn first_9(r: *mut [(); 3]) -> () {
    return (*r)[(0_()) as usize];
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder_int_ {
    pub v: (),
}
impl Holder_int_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    let mut a: () = 0_();
    unimplemented!();
    unimplemented!();
    let mut b: Bar = Bar::new(None);
    unimplemented!();
    unimplemented!();
    let mut arr: [Bar; 3] = [Bar::new(None), Bar::new(None), Bar::new(None)];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h: Holder_int_ = Holder_int_::new({ 4_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_2);
    std::cell::LazyCell::force(&*&raw const global_bar_6);
}
