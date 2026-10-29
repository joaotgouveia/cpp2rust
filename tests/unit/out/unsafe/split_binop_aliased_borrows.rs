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
    let mut v: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    let mut p: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
        (unsafe { v.begin_26() });
    let r: *const () = &v[(1_()) as usize];
    (*p) = (*r);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
