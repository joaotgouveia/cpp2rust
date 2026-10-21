extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(true, "Small<int>");
const _: () = assert!(
    ((::std::mem::size_of::<()>()) == (4_())),
    "sizeof(int) == 4"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Sized {}
impl Sized {
    pub unsafe fn size(&mut self) -> () {
        return 4_();
    }
}
pub unsafe fn is_small_0() -> () {
    return true;
}
pub unsafe fn is_small_1() -> () {
    return false;
}
pub unsafe fn has_size_2() -> () {
    return true;
}
pub unsafe fn has_size_3() -> () {
    return false;
}
pub unsafe fn pick_4(mut x: ()) -> () {
    {
        return 1_();
    }
    return 2_();
}
pub unsafe fn pick_5(mut x: ()) -> () {
    {}
    return 2_();
}
pub unsafe fn pick_6(mut x: ()) -> () {
    {}
    return 2_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    const _: () = assert!(!(false), "!Small<long>");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
