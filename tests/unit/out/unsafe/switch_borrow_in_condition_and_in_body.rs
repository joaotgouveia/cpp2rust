extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn borrow_in_condition_and_in_body_0(mut x: i32) -> i32 {
    switch!(match x {
        __v if __v == 0 => {}
        _ => {
            return ((x) + (1));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { borrow_in_condition_and_in_body_0(0) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"borrow_in_condition_and_in_body(0) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { borrow_in_condition_and_in_body_0(1) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"borrow_in_condition_and_in_body(1) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
