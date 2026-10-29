extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut bytes: std_initializer_list_int_) -> () {
    let mut buf: *mut std_vector_int__std_allocator_int__ = (Box::leak(Box::new(
        std_vector_int__std_allocator_int__::new_1({ bytes }, None),
    ))
        as *mut std_vector_int__std_allocator_int__);
    let mut n: () = (unsafe { bytes.size() });
    {
        let __p = buf;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return n;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    let mut v: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    unimplemented!();
    unimplemented!();
    let mut l: std_initializer_list_int_ = unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
