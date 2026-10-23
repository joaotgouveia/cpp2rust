extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut myRope: __gnu_cxx_rope_char__std_allocator_char__ =
        <__gnu_cxx_rope_char__std_allocator_char__>::default();
    myRope = __gnu_cxx_rope_char__std_allocator_char__::new_1({ c"1234567890".as_ptr() }, None);
    let mut buffer: [(); 100] = [unimplemented!(); 100];
    (unsafe { myRope.copy_58(1_(), 1_(), buffer.as_mut_ptr()) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
