extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn compound_case_body_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 1 => {
                let mut y: i32 = 10;
                let mut z: i32 = 20;
                r = ((y) + (z));
                break 'switch;
            }
            __v if __v == 2 => {
                let mut y: i32 = 100;
                r = ((y) - (1));
                break 'switch;
            }
            _ => {
                r = -1_i32;
                break 'switch;
            }
        }
    };
    return r;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { compound_case_body_0(1) }) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"compound_case_body(1) == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { compound_case_body_0(2) }) == (99)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"compound_case_body(2) == 99".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { compound_case_body_0(9) }) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"compound_case_body(9) == -1".as_ptr(),
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
