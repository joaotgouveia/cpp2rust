extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    {
        let mut list1: () = unimplemented!();
        let mut list2: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        let mut node_of_interest: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut list1: () = unimplemented!();
        let mut list2: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        let mut node_of_interest: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
