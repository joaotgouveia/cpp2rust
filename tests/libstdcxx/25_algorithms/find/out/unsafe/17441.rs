extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn find_0(mut first: *mut (), mut _a1: *mut (), _a2: *const (), mut _a3: ()) -> *mut () {
    return first;
}
pub unsafe fn test01_1() {
    let mut a: () = unimplemented!();
    let mut i: () = unimplemented!();
    (unsafe {
        let _first: *mut () = (&mut i as *mut ());
        let _arg1: *mut () = (&mut i as *mut ());
        let mut _arg2: () = 1_();
        find_0(_first, _arg1, &mut _arg2, unimplemented!())
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
