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
    let mut vec_: std_vector_unsigned_int__std_allocator_unsigned_int__ =
        <std_vector_unsigned_int__std_allocator_unsigned_int__>::default();
    (unsafe {
        let mut ___x: () = 67305985_();
        vec_.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 134678021_();
        vec_.push_back_58(&mut ___x)
    });
    let mut bytes: *mut () = ((unsafe { vec_.data_55() }) as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*bytes.offset((4_()) as isize)) = 255_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
