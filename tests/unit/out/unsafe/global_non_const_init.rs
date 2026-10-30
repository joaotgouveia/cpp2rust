extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn next_0() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_1)).prefix_inc();
}
static mut counter_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn marker_2(mut tag: ()) -> () {
    return (((tag) << (3_())) | (2_()));
}
pub static mut signature_3: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [(unsafe { marker_2(1_()) }), 4_(), ('B' as ())] });
pub static mut single_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { marker_2(2_()) }) });
pub static mut from_call_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { next_0() }) });
pub static mut depends_on_call_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut from_call_5)) + (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Ctor {
    pub v: (),
}
impl Ctor {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v: (unsafe { next_0() }),
        };
        this
    }
    pub unsafe fn new_2(mut x: ()) -> Self {
        let mut this = Self { v: x };
        this
    }
}
impl Default for Ctor {
    fn default() -> Self {
        unsafe { Ctor::new_1() }
    }
}
pub static mut default_ctor_7: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_1() });
pub static mut arg_ctor_8: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_2({ 7_() }) });
pub static mut str_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut inline_member_11: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_2({ 5_() }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {}
pub static mut member_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { next_0() }) });
pub unsafe fn local_static_12() -> () {
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut once_13))
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut local_ctor_14)).v));
}
static mut once_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { next_0() }) });
static mut local_ctor_14: std::cell::LazyCell<Ctor> =
    std::cell::LazyCell::new(|| unsafe { Ctor::new_2({ 3_() }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Singleton {
    pub hits: (),
}
impl Singleton {
    pub unsafe fn new() -> Self {
        let mut this = Self { hits: 0_() };
        this
    }
    pub unsafe fn instance() -> *mut Singleton {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut s_15));
    }
}
impl Default for Singleton {
    fn default() -> Self {
        unsafe { Singleton::new() }
    }
}
static mut s_15: std::cell::LazyCell<Singleton> =
    std::cell::LazyCell::new(|| unsafe { Singleton::new() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
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
    unimplemented!();
    unimplemented!();
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const signature_3);
    std::cell::LazyCell::force(&*&raw const single_4);
    std::cell::LazyCell::force(&*&raw const from_call_5);
    std::cell::LazyCell::force(&*&raw const depends_on_call_6);
    std::cell::LazyCell::force(&*&raw const default_ctor_7);
    std::cell::LazyCell::force(&*&raw const arg_ctor_8);
    std::cell::LazyCell::force(&*&raw const str_9);
    std::cell::LazyCell::force(&*&raw const inline_member_11);
    std::cell::LazyCell::force(&*&raw const member_10);
}
