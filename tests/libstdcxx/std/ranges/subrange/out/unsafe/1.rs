extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    let mut b: [*mut (); 3] = [
        (&mut a[(2_()) as usize] as *mut ()),
        (&mut a[(0_()) as usize] as *mut ()),
        (&mut a[(1_()) as usize] as *mut ()),
    ];
    let mut c: () = unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
