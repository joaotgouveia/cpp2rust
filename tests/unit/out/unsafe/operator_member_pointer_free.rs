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
pub struct Inner {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub data: [(); 3],
    pub inner: Inner,
}
pub unsafe fn operator_deref_0(s: *mut S) -> *mut Inner {
    return &mut (*s).inner;
}
pub unsafe fn operator_addr_1(s: *mut S) -> *mut () {
    return (&mut (*s).data[(unimplemented!()) as usize] as *mut ());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S {
        data: [1_(), 2_(), 3_()],
        inner: Inner { x: 9_() },
    };
    unimplemented!();
    (*(unsafe {
        let _s: *mut S = &mut s;
        operator_deref_0(_s)
    }))
    .x = 10_();
    unimplemented!();
    let mut p: *mut () = (unsafe {
        let _s: *mut S = &mut s;
        operator_addr_1(_s)
    });
    unimplemented!();
    (*p) = 5_();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
