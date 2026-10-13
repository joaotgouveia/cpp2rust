extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cases_and_default_stacked_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 42;
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
    (if ((unsafe { cases_and_default_stacked_0(1) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"cases_and_default_stacked(1) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { cases_and_default_stacked_0(2) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"cases_and_default_stacked(2) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { cases_and_default_stacked_0(3) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"cases_and_default_stacked(3) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { cases_and_default_stacked_0(99) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"cases_and_default_stacked(99) == 42".as_ptr(),
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
