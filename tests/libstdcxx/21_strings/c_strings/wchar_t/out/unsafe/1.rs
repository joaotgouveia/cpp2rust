extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut c: i32 = (97 as i32);
    let cc: i32 = (98 as i32);
    let mut c1: *mut i32 = (&mut c as *mut i32);
    let mut cc1: *const i32 = (&cc as *const i32);
    let mut ccarray1: *const i32 = [
        115 as i32, 97 as i32, 110 as i32, 32 as i32, 102 as i32, 114 as i32, 97 as i32,
        110 as i32, 99 as i32, 105 as i32, 115 as i32, 99 as i32, 111 as i32, 32 as i32,
        114 as i32, 111 as i32, 111 as i32, 102 as i32, 32 as i32, 103 as i32, 97 as i32,
        114 as i32, 100 as i32, 101 as i32, 110 as i32, 32 as i32, 105 as i32, 110 as i32,
        115 as i32, 112 as i32, 101 as i32, 99 as i32, 116 as i32, 111 as i32, 114 as i32,
        115 as i32, 0 as i32,
    ]
    .as_ptr();
    let mut ccarray2: *const i32 = [
        115 as i32, 97 as i32, 110 as i32, 32 as i32, 102 as i32, 114 as i32, 97 as i32,
        110 as i32, 99 as i32, 105 as i32, 115 as i32, 99 as i32, 111 as i32, 32 as i32,
        115 as i32, 117 as i32, 110 as i32, 110 as i32, 121 as i32, 45 as i32, 100 as i32,
        97 as i32, 121 as i32, 32 as i32, 112 as i32, 97 as i32, 114 as i32, 107 as i32, 32 as i32,
        105 as i32, 110 as i32, 115 as i32, 112 as i32, 101 as i32, 99 as i32, 116 as i32,
        111 as i32, 114 as i32, 115 as i32, 0 as i32,
    ]
    .as_ptr();
    let mut carray: [i32; 50] = [0_i32; 50];
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
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
