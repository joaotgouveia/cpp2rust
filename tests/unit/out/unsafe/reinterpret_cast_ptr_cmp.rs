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
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 42;
    let mut ptr1: *mut i32 = (&mut i1 as *mut i32);
    let mut ptr2: *mut libc::c_char = ((&mut i1 as *mut i32) as *mut libc::c_char);
    let mut vptr1: *mut ::libc::c_void = (ptr1 as *mut ::libc::c_void);
    let mut vptr2: *mut ::libc::c_void = (ptr2 as *mut ::libc::c_void);
    (if ((vptr1) == (vptr2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"vptr1 == vptr2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
