extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo1_0() {
    let mut v1: __gnu_cxx_throw_value_limit = <__gnu_cxx_throw_value_limit>::default();
    let mut v2: __gnu_cxx_throw_value_limit = v1.clone();
    let mut v3: __gnu_cxx_throw_value_limit = <__gnu_cxx_throw_value_limit>::default();
}
pub unsafe fn foo2_1() -> () {
    let mut b: () = unimplemented!();
    return b;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { foo1_0() });
    (unsafe { foo2_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
