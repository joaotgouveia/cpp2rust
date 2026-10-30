extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> () {
    return true;
}
pub unsafe fn bar_1() -> () {
    return true;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i1: () = 0_();
    let mut i2: () = 1_();
    write!((*unimplemented!()), "{:}\n", (true as u8),);
    write!((*unimplemented!()), "{:}\n", (false as u8),);
    write!((*unimplemented!()), "{:}\n", (((i1) != (i2)) as u8),);
    write!((*unimplemented!()), "{:}\n", (((i1) == (i2)) as u8),);
    write!((*unimplemented!()), "{:}\n", ((unsafe { foo_0() }) as u8),);
    write!((*unimplemented!()), "{:}\n", ((unsafe { bar_1() }) as u8),);
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
