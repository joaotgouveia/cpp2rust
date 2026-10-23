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
    let mut e: () = unimplemented!();
    let mut str: *const () = unimplemented!();
    let mut limit_l: () = unimplemented!();
    let mut r: () = unimplemented!();
    let mut cp: *const () = (Box::leak(Box::new(unimplemented!())) as *mut ()).cast_const();
    {
        let __p = cp;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p.cast_mut()))
        }
    };
    let mut b: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut value_0));
    let mut ilisti: () = unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
