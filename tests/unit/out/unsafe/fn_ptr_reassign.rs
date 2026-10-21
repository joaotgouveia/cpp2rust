extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_0(mut a: (), mut b: ()) -> () {
    return ((a) + (b));
}
pub unsafe fn sub_1(mut a: (), mut b: ()) -> () {
    return ((a) - (b));
}
pub unsafe fn mul_2(mut a: (), mut b: ()) -> () {
    return ((a) * (b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut fn_: Option<unsafe fn((), ()) -> ()> = Some(add_0);
    unimplemented!();
    fn_ = Some(sub_1);
    unimplemented!();
    fn_ = Some(mul_2);
    unimplemented!();
    fn_ = None;
    unimplemented!();
    fn_ = Some(add_0);
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
