extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn mixed_literal_cases_0(mut x: i32) -> i32 {
    'switch: {
        match { x } {
            __v if __v == -1_i32 => {
                return 1;
            }
            __v if __v == 16 => {
                return 2;
            }
            __v if __v == 65152 => {
                return 3;
            }
            __v if __v == -255_i32 => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { mixed_literal_cases_0(-1_i32) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"mixed_literal_cases(-1) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { mixed_literal_cases_0(16) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"mixed_literal_cases(0x10) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { mixed_literal_cases_0(65152) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"mixed_literal_cases(0xFE80) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { mixed_literal_cases_0(-255_i32) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"mixed_literal_cases(-0xFF) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { mixed_literal_cases_0(7) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"mixed_literal_cases(7) == 0".as_ptr(),
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
