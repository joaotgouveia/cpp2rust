extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut a: [i32; 3] = [1, 2, 3];
    let mut b: [*mut i32; 3] = [
        (&mut a[(2) as usize] as *mut i32),
        (&mut a[(0) as usize] as *mut i32),
        (&mut a[(1) as usize] as *mut i32),
    ];
    let mut c: () = unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
