extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn find_0(
    mut first: *mut i32,
    mut _a1: *mut i32,
    _a2: *const i32,
    mut _a3: (),
) -> *mut i32 {
    return first;
}
pub unsafe fn test01_1() {
    let mut a: () = unimplemented!();
    let mut i: i32 = 0_i32;
    (unsafe {
        let _first: *mut i32 = (&mut i as *mut i32);
        let _arg1: *mut i32 = (&mut i as *mut i32);
        let mut _arg2: i32 = 1;
        find_0(_first, _arg1, &mut _arg2, unimplemented!())
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
