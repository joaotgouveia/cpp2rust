extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn escape_0(mut ptr: *mut ()) {
    {
        let __p = ptr;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut alloc: *mut () = (Box::leak(Box::new(1_())) as *mut ());
    (unsafe { escape_0(alloc) });
    {
        let __p = alloc;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
