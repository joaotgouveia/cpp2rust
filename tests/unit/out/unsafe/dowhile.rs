extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dowhile_0(mut x: i32) -> i32 {
    let mut __do_while = true;
    'loop_: while __do_while || ((x) <= (200)) {
        __do_while = false;
        x += 1;
        let mut __do_while = true;
        'loop_: while __do_while || ((x) <= (100)) {
            __do_while = false;
            x += 1;
            x += 1;
        }
        x += 1;
    }
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { dowhile_0(0) }) == (202)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"dowhile(0) == 202".as_ptr(),
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
