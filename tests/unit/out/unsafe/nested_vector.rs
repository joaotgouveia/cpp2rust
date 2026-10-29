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
    let mut v1 : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() ;
    let mut v2: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    (unsafe { v1.push_back_57(&v2) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
