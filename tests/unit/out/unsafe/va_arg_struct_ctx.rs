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
pub struct context {
    pub verbose: (),
    pub last_error: (),
}
pub unsafe fn set_error_0(mut ctx: *mut context, mut fmt: *const (), __args: Box<[VaArg]>) {
    if ((*ctx).verbose != 0) {
        let mut ap: *const VaArg = ::std::ptr::null();
        ap = __args.as_ptr();
        (*ctx).last_error = ap.arg::<()>();
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union value {
    pub i: (),
    pub l: (),
}
impl Default for value {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn pick_1(mut use_long: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut v: value = ap.arg::<value>();
    if (use_long != 0) {
        return v.l;
    }
    return v.i;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut ctx: context = <context>::default();
    ctx.verbose = 1_();
    ctx.last_error = 0_();
    (unsafe {
        set_error_0(
            (&mut ctx as *mut context),
            (c"error %d".as_ptr().cast_mut()).cast_const(),
            Box::new([(42_()).into()]),
        )
    });
    unimplemented!();
    ctx.verbose = 0_();
    (unsafe {
        set_error_0(
            (&mut ctx as *mut context),
            (c"error %d".as_ptr().cast_mut()).cast_const(),
            Box::new([(99_()).into()]),
        )
    });
    unimplemented!();
    let mut v: value = <value>::default();
    v.l = ((1_()) << (40_()));
    unimplemented!();
    v.i = 7_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
