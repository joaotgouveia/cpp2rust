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
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: () = 5_();
    let mut v5: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut m: () = unimplemented!();
    (*unimplemented!()) = 10_();
    (*unimplemented!()) = 20_();
    (*unimplemented!()) = 30_();
    let mut v6: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
