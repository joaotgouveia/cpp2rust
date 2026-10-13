extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_default_0(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 7 => {
            if (flag != 0) {
                r = 100;
                break;
            };
        }
        _ => {
            r = 42;
            break;
        }
    });
    return r;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { fallthrough_default_0(7, 0) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"fallthrough_default(7, 0) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { fallthrough_default_0(7, 1) }) == (100)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"fallthrough_default(7, 1) == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { fallthrough_default_0(99, 0) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"fallthrough_default(99, 0) == 42".as_ptr(),
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
