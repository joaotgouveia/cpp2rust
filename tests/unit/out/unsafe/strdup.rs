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
    pub name: *mut (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut d: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = (c"world".as_ptr().cast_mut()).cast_const();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    let mut d2: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d3: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d4: *mut () = std::ptr::null_mut();
    d4 = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut rec: record = record {
        name: std::ptr::null_mut(),
    };
    let mut r: *mut record = (&mut rec as *mut record);
    (*r).name = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
