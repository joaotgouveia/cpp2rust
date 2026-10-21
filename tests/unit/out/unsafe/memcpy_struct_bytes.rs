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
pub struct point {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut src: point = point {
        x: (3_() as i32),
        y: (7_() as i32),
    };
    let mut buf: [(); 8] = [unimplemented!(); 8];
    (unsafe {
        let ___dest: () = (buf.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[(); 8]>();
        memcpy_0(___dest, ((&mut src as *mut point) as ()), ___n)
    });
    let mut dst: point = <point>::default();
    (unsafe {
        let ___dest: () = ((&mut dst as *mut point) as ());
        let ___n: () = ::std::mem::size_of::<point>();
        memcpy_0(___dest, (buf.as_mut_ptr() as ()), ___n)
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
