extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_on_assignment_0(mut x: i32) -> i32 {
    let mut y: i32 = 0;
    let mut r: i32 = 0;
    'switch: {
        match {
            {
                y = ((x) + (1));
                y
            }
        } {
            __v if __v == 1 => {
                r = 10;
                break 'switch;
            }
            __v if __v == 2 => {
                r = 20;
                break 'switch;
            }
            _ => {
                r = y;
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
    (if ((unsafe { switch_on_assignment_0(0) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_on_assignment(0) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_on_assignment_0(1) }) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_on_assignment(1) == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_on_assignment_0(9) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_on_assignment(9) == 10".as_ptr(),
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
