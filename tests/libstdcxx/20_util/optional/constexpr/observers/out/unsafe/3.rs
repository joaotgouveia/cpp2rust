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
pub struct value_type {
    pub i: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let o: () = value_type { i: 51_() };
    const _: () = assert!(true, "o.value().i == 51");;
    const _: () = assert!(true, "o.value().i == (*o).i");;
    const _: () = assert!(true, "&o.value().i == &(*o).i");;
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
