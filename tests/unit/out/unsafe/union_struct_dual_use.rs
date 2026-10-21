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
    pub a: (),
    pub b: (),
}
pub unsafe fn sum_inner_0(mut i: *mut Inner) -> () {
    return (((*i).a) + ((*i).b));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub inner: Inner,
    pub raw_: [(); 16],
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub u: anon_1,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut standalone: Inner = <Inner>::default();
    standalone.a = 3_();
    standalone.b = 4_();
    unimplemented!();
    let mut outer: Outer = <Outer>::default();
    (unsafe {
        let ___s: () = ((&mut outer as *mut Outer) as ());
        let ___n: () = ::std::mem::size_of::<Outer>();
        memset_2(___s, 0_(), ___n)
    });
    outer.u.inner.a = 3_();
    outer.u.inner.b = 4_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
