extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_errno_0() {
    (*unimplemented!()) = 0_();
    unimplemented!();
    (*unimplemented!()) = 42_();
    unimplemented!();
    let mut saved: () = (*unimplemented!());
    unimplemented!();
    (*unimplemented!()) = 0_();
}
pub unsafe fn test_errno_preserved_across_strdup_1() {
    (*unimplemented!()) = 99_();
    let mut d: *mut () = (unsafe { strdup_2((c"hello".as_ptr().cast_mut()).cast_const()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 0_();
}
pub unsafe fn test_errno_from_fseek_3() {
    (*unimplemented!()) = 0_();
    let mut r: () = libc::fseek(
        libcc2rs::stdin_unsafe(),
        0_() as ::libc::c_long,
        ::libc::SEEK_SET,
    );
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 0_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_errno_0() });
    (unsafe { test_errno_preserved_across_strdup_1() });
    (unsafe { test_errno_from_fseek_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
