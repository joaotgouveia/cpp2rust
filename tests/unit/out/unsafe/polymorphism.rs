extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait Animal {
    unsafe fn bark(&self) -> ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Dog {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cat {}
impl Cat {
    unsafe fn meow(&self) -> () {
        return true;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut dog: Dog = <Dog>::default();
    let mut animal: *mut dyn Animal = (&mut dog as *mut Dog);
    let mut eat1: () = (unsafe { (*(animal).cast_const()).bark() });
    let mut cat: Cat = <Cat>::default();
    animal = (&mut cat as *mut Cat);
    let mut eat2: () = (unsafe { (*(animal).cast_const()).bark() });
    unimplemented!();
    return 0_();
}
unsafe impl Animal for Cat {
    unsafe fn bark(&self) -> () {
        return false;
    }
}
unsafe impl Animal for Dog {
    unsafe fn bark(&self) -> () {
        return true;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
