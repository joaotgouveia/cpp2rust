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
pub struct S {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: [S; 1] = [<S>::default(); 1];
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut dv: std___debug_vector_S__std_allocator_S__ =
        std___debug_vector_S__std_allocator_S__::new_1({ 1_() }, None);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
