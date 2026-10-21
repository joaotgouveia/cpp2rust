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
    let mut ops: [Option<unsafe fn((), ()) -> ()>; 3] = [Some(add_0), Some(sub_1), Some(mul_2)];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
