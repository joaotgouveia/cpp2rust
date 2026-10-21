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
pub struct X {
    pub x: (),
}
pub unsafe fn func_0() -> () {
    return 42_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = unimplemented!();
    let mut p: *mut () = std::ptr::null_mut();
    let g: *mut () = &mut x;
    let mut q: *mut () = (&mut x as *mut ());
    let mut z: *mut () = p;
    let mut xx: X = <X>::default();
    let mut zz: *mut X = (&mut xx as *mut X);
    xx.x = 1_();
    q = (&mut xx.x as *mut ());
    q = (&mut (*zz).x as *mut ());
    (*zz).x = 2_();
    let mut ww: X = xx;
    ww = xx;
    let mut aa: () = (unsafe { func_0() });
    aa = (unsafe { func_0() });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
