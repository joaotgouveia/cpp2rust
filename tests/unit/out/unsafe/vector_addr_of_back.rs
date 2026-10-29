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
    let mut outer : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() ;
    let mut inner: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe { outer.push_back_57(&inner) });
    let mut sink: *mut std_vector_int__std_allocator_int__ = (unsafe { outer.back_53() });
    unimplemented!();
    let mut p : *mut std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = ( & mut outer as *mut std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ ) ;
    sink = (unsafe { (*p).back_53() });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
