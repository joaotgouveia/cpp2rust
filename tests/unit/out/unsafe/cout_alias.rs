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
    let mut cond: () = true;
    let mut os1: () = if cond {
        (&raw mut (*unimplemented!()) as ())
    } else {
        (&raw mut (*unimplemented!()) as ())
    };
    write!((*os1), "hello\n",);
    let os2: () = if cond {
        (&raw mut (*unimplemented!()) as ())
    } else {
        (&raw mut (*unimplemented!()) as ())
    };
    write!((*os2), "hello\n",);
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
