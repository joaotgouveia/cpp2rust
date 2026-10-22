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
    let mut p: *mut u32 = (Box::leak(Box::new(67305985_u32)) as *mut u32);
    let mut bytes: *mut u8 = (p as *mut u8);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*bytes.offset((0_()) as isize)) = 16_u8;
    unimplemented!();
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
