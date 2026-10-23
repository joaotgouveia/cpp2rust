extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut v: std___debug_vector_int__std_allocator_int__ =
        std___debug_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    unimplemented!();
}
pub unsafe fn test02_1() {
    let mut l: std___debug_list_int__std_allocator_int__ =
        std___debug_list_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    (unsafe { test02_1() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
