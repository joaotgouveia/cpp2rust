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
pub struct header {
    pub tag: (),
    pub size: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct view {
    pub tag: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let text: [(); 3] = std::mem::transmute(*b"hi\0");
    let mut cp: *const () = text.as_ptr();
    let mut u: *mut () = (cp as *mut ());
    unimplemented!();
    unimplemented!();
    let mut h: header = header {
        tag: 7_(),
        size: 32_(),
    };
    let mut hp: *mut header = (&mut h as *mut header);
    let mut v: *mut view = (hp as *mut view);
    unimplemented!();
    let mut data: [(); 3] = std::mem::transmute(*b"hi\0");
    let mut vp: () = (data.as_mut_ptr() as ());
    let mut n: () = 2_();
    let mut sel: *mut () = (if ((((n) < (100_())) as ()) != 0) {
        vp
    } else {
        (0_() as ())
    } as *mut ());
    unimplemented!();
    unimplemented!();
    n = 200_();
    sel = (if ((((n) < (100_())) as ()) != 0) {
        vp
    } else {
        (0_() as ())
    } as *mut ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
