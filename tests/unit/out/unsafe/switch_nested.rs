extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn nested_0(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { a } {
            __v if __v == 1 => {
                'switch: {
                    match { b } {
                        __v if __v == 10 => {
                            r = 11;
                            break 'switch;
                        }
                        __v if __v == 20 => {
                            r = 12;
                            break 'switch;
                        }
                        _ => {
                            r = 13;
                            break 'switch;
                        }
                    }
                };
                r += 1;
                break 'switch;
            }
            __v if __v == 2 => {
                r = 2;
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
    (if ((unsafe { nested_0(1, 10) }) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"nested(1, 10) == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { nested_0(1, 99) }) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"nested(1, 99) == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { nested_0(2, 0) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"nested(2, 0) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { nested_0(3, 3) }) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"nested(3, 3) == -1".as_ptr(),
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
