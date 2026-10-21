extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = -1_();
    let mut pp: anon_0 = <anon_0>::default();
    pp.as_signed = (&mut x as *mut ());
    (*pp.as_unsigned) = 42_();
    unimplemented!();
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub as_unsigned: *mut (),
    pub as_signed: *mut (),
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
