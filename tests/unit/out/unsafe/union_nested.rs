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
pub struct record {
    pub code: u16,
    pub pad: [(); 14],
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub h: record,
    pub raw_: [(); 128],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct inner {
    pub view: anon_0,
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub h: record,
    pub nested: inner,
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub kind: (),
    pub level: (),
    pub variant: (),
    pub len: (),
    pub body: anon_1,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut ex: Outer = <Outer>::default();
    (unsafe {
        let ___s: () = ((&mut ex as *mut Outer) as ());
        let ___n: () = ::std::mem::size_of::<Outer>();
        memset_2(___s, 0_(), ___n)
    });
    ex.kind = 2_();
    ex.level = 1_();
    ex.variant = 6_();
    ex.len = ::std::mem::size_of::<record>();
    ex.body.h.code = 2_u16;
    ex.body.h.pad[(0_()) as usize] = ('X' as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
