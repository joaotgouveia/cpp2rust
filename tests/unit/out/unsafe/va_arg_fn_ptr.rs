extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn square_0(mut x: ()) -> () {
    return ((x) * (x));
}
pub unsafe fn negate_1(mut x: ()) -> () {
    return -x;
}
pub unsafe fn add_2(mut a: (), mut b: ()) -> () {
    return ((a) + (b));
}
pub unsafe fn apply_unary_3(mut x: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut fn_: Option<unsafe fn(()) -> ()> = std::mem::transmute::<
        *mut ::libc::c_void,
        Option<unsafe fn(()) -> ()>,
    >(ap.arg::<*mut ::libc::c_void>());
    let mut result: () = (unsafe { (fn_).unwrap()(x) });
    return result;
}
pub unsafe fn apply_binary_4(mut a: (), mut b: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut fn_: Option<unsafe fn((), ()) -> ()> = std::mem::transmute::<
        *mut ::libc::c_void,
        Option<unsafe fn((), ()) -> ()>,
    >(ap.arg::<*mut ::libc::c_void>());
    let mut result: () = (unsafe { (fn_).unwrap()(a, b) });
    return result;
}
pub unsafe fn not_supported_5(
    mut ctx: (),
    mut fn_: Option<unsafe fn(()) -> ()>,
    mut extra: (),
) -> () {
    &(ctx);
    &(fn_);
    &(extra);
    return -3_();
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
    unimplemented!();
    let mut dummy: () = 0_();
    unimplemented!();
    let mut vu: Option<unsafe fn((), Box<[VaArg]>) -> ()> = Some(apply_unary_3);
    unimplemented!();
    let mut vb: Option<unsafe fn((), (), Box<[VaArg]>) -> ()> = std::mem::transmute::<
        Option<unsafe fn((), Box<[VaArg]>) -> ()>,
        Option<unsafe fn((), (), Box<[VaArg]>) -> ()>,
    >(Some(apply_unary_3));
    vu = std::mem::transmute::<
        Option<unsafe fn((), (), Box<[VaArg]>) -> ()>,
        Option<unsafe fn((), Box<[VaArg]>) -> ()>,
    >(vb);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
