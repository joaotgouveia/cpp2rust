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
    let mut pw: *mut () = unimplemented!();
    if !(!(pw).is_null()) {
        return unimplemented!();
    }
    let mut home: *mut () = (*pw).pw_dir;
    let mut d: *mut () = unimplemented!();
    let mut dname: *mut () = (*d).d_name.as_mut_ptr();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
