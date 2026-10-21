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
    let mut p: *mut () = (Box::leak(Box::new(67305985_())) as *mut ());
    let mut bytes: *mut () = (p as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*bytes.offset((unimplemented!()) as isize)) = 16_();
    unimplemented!();
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
