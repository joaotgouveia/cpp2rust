extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut c: () = (97 as ());
    let cc: () = (98 as ());
    let mut c1: *mut () = (&mut c as *mut ());
    let mut cc1: *const () = (&cc as *const ());
    let mut ccarray1: *const () = [
        115 as (), 97 as (), 110 as (), 32 as (), 102 as (), 114 as (), 97 as (), 110 as (),
        99 as (), 105 as (), 115 as (), 99 as (), 111 as (), 32 as (), 114 as (), 111 as (),
        111 as (), 102 as (), 32 as (), 103 as (), 97 as (), 114 as (), 100 as (), 101 as (),
        110 as (), 32 as (), 105 as (), 110 as (), 115 as (), 112 as (), 101 as (), 99 as (),
        116 as (), 111 as (), 114 as (), 115 as (), 0 as (),
    ]
    .as_ptr();
    let mut ccarray2: *const () = [
        115 as (), 97 as (), 110 as (), 32 as (), 102 as (), 114 as (), 97 as (), 110 as (),
        99 as (), 105 as (), 115 as (), 99 as (), 111 as (), 32 as (), 115 as (), 117 as (),
        110 as (), 110 as (), 121 as (), 45 as (), 100 as (), 97 as (), 121 as (), 32 as (),
        112 as (), 97 as (), 114 as (), 107 as (), 32 as (), 105 as (), 110 as (), 115 as (),
        112 as (), 101 as (), 99 as (), 116 as (), 111 as (), 114 as (), 115 as (), 0 as (),
    ]
    .as_ptr();
    let mut carray: [(); 50] = [unimplemented!(); 50];
    unimplemented!();
    cc1 = (unimplemented!()).cast_const();
    c1 = unimplemented!();
    cc1 = (unimplemented!()).cast_const();
    c1 = unimplemented!();
    cc1 = (unimplemented!()).cast_const();
    c1 = unimplemented!();
    cc1 = (unimplemented!()).cast_const();
    c1 = unimplemented!();
    cc1 = (unimplemented!()).cast_const();
    c1 = unimplemented!();
    cc1 = cc1;
    c1 = c1;
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
