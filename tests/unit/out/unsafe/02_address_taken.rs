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
    let mut b: () = 2_();
    let mut b_ptr: *mut () = (&mut b as *mut ());
    (*b_ptr) = 3_();
    let mut b_ptr_ptr: *mut *mut () = (&mut b_ptr as *mut *mut ());
    (*(*b_ptr_ptr)) = 4_();
    (*b_ptr) = (*(*b_ptr_ptr));
    let mut offset: () = ((b_ptr as usize - b_ptr as usize) / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
