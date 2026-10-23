extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct moveable {}
impl moveable {
    pub unsafe fn move_from(_a0: *mut moveable) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn to_(&mut self) -> () {
        return 0_();
    }
}
impl Default for moveable {
    fn default() -> Self {
        unsafe { moveable::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut the_map: () = unimplemented!();
    return (unsafe { moveable::to_(&mut (*unimplemented!())) });
}
pub unsafe fn __cpp2rust_init_globals() {}
