extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut val: () = 42_();
    let mut original: *mut () = (&mut val as *mut ());
    let mut as_u16: *mut () = (original as *mut ());
    let mut back: *mut () = (as_u16 as *mut ());
    unimplemented!();
    unimplemented!();
    let mut as_u8: *mut () = (original as *mut ());
    let mut back2: *mut () = (as_u8 as *mut ());
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
