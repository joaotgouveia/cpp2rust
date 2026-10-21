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
pub struct Foo {
    pub x1: (),
    pub x2: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: [Foo; 2] = [Foo { x1: 1_(), x2: 2_() }, Foo { x1: 3_(), x2: 4_() }];
    let mut p1: *mut () = (&mut arr[(1_()) as usize].x1 as *mut ());
    let mut a: () = (*p1);
    let mut p2: *mut Foo = (&mut arr[(unimplemented!()) as usize] as *mut Foo);
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
