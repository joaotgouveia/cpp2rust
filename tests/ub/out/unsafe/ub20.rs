extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut single: *mut ()) {
    {
        let __p = single;
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
    let mut x: *mut () =
        Box::leak((0..10_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (unsafe { foo_0(x) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
