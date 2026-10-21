extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn bar_0() -> () {
    return 1_();
}
pub unsafe fn foo_1(x: *mut ()) -> *mut () {
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 5_();
    let mut y: () = (*(unsafe { foo_1(&mut x) }));
    let z: *mut () = (unsafe { foo_1(&mut x) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
