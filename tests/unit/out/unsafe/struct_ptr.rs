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
pub struct XX {
    pub x: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut obj: XX = <XX>::default();
    let mut ptr: *mut XX = (&mut obj as *mut XX);
    (*ptr).x = 2_();
    let mut c: () = false;
    let mut r: () = if c { obj.x } else { (*ptr).x };
    let mut p: *mut () = (&mut obj.x as *mut ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
