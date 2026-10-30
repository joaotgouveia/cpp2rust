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
    let a: *const () = (unsafe {
        let mut ___a: () = 1_();
        let mut ___b: () = 2_();
        min_0(&mut ___a, &mut ___b)
    });
    let b: *const () = (unsafe {
        let mut ___a: () = 1_();
        let mut ___b: () = 2_();
        max_1(&mut ___a, &mut ___b)
    });
    return ((*a) == (*b));
}
pub unsafe fn __cpp2rust_init_globals() {}
