extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn square_0(mut x: ()) -> () {
    return ((x) * (x));
}
pub unsafe fn twice_1(mut x: ()) -> () {
    return ((2_()) * (x));
}
pub unsafe fn call_ref_2(f: Option<unsafe fn(()) -> ()>, mut x: ()) -> () {
    return (unsafe { (f).unwrap()(x) });
}
pub unsafe fn call_deduced_3(f: Option<unsafe fn(()) -> ()>, mut x: ()) -> () {
    return (unsafe { (f).unwrap()(x) });
}
pub unsafe fn call_forwarded_4(f: Option<unsafe fn(()) -> ()>, mut x: ()) -> () {
    return (unsafe { (f).unwrap()(x) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub f: Option<unsafe fn(()) -> ()>,
}
impl Holder {
    pub unsafe fn run(&self, mut x: ()) -> () {
        return (unsafe { (self.f).unwrap()(x) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    let r: Option<unsafe fn(()) -> ()> = Some(square_0);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h: Holder = Holder { f: Some(twice_1) };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
