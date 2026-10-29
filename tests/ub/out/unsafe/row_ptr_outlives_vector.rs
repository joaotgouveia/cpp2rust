extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn second_row_0() -> *mut () {
    let mut rows : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ :: new_1 ( { unimplemented!() } , None , )  ;
    return (unsafe { rows[(1_()) as usize].data_55() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: *mut () = (unsafe { second_row_0() });
    return (*p.offset((0_()) as isize));
}
pub unsafe fn __cpp2rust_init_globals() {}
