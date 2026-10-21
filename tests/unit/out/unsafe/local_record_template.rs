extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(mut t: Local_1) -> () {
    return t.x;
}
pub unsafe fn get_2(mut t: Local_3) -> () {
    return t.x;
}
pub unsafe fn get_4(mut t: Local_5) -> () {
    return t.x;
}
pub unsafe fn get_6(mut t: Local_7) -> () {
    return t.x;
}
pub unsafe fn twice_8(mut t: Local_3) -> () {
    return ((t.x) * (2_()));
}
pub unsafe fn wrap_9(mut v: ()) -> () {
    let mut l: Local_5 = Local_5 { x: v };
    return (unsafe { get_4(l) });
}
pub unsafe fn wrap_10(mut v: ()) -> () {
    let mut l: Local_7 = Local_7 { x: v };
    return (unsafe { get_6(l) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_5 {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_7 {
    pub x: (),
}
pub unsafe fn other_11() -> () {
    let mut l: Local_1 = Local_1 { x: 3_(), y: 4_() };
    return ((unsafe { get_0(l) }) + (l.y));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_1 {
    pub x: (),
    pub y: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut l: Local_3 = Local_3 { x: 7_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_3 {
    pub x: (),
}
pub unsafe fn __cpp2rust_init_globals() {}
