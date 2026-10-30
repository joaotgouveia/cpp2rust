extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct S {
    pub r: *const (),
}
impl S {
    pub unsafe fn new(x: *const ()) -> Self {
        let mut this = Self { r: x };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = {
        let mut __tmp_0: () = 5_();
        S::new({ &mut __tmp_0 })
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
