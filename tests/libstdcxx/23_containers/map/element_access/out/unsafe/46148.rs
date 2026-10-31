extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_into(i32, to_i32)]
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
    pub unsafe fn to_i32(&mut self) -> i32 {
        return 0;
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
unsafe fn main_0() -> i32 {
    let mut the_map: UnsafeMap<bool, (bool, moveable)> = UnsafeMap::new();
    return (unsafe {
        moveable::to_i32(&mut (*the_map.subscript_move(&mut true, |__k| cpp_move_unsafe!(__k))))
    });
}
pub unsafe fn __cpp2rust_init_globals() {}
