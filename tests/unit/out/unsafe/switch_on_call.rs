extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut v: i32) -> i32 {
    return ((v) * (2));
}
pub unsafe fn switch_on_call_1(mut x: i32) -> i32 {
    'switch: {
        match { (unsafe { double_it_0(x) }) } {
            __v if __v == 0 => {
                return 100;
            }
            __v if __v == 2 => {
                return 200;
            }
            __v if __v == 4 => {
                return 400;
            }
            _ => {
                return -1_i32;
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
    (if ((unsafe { switch_on_call_1(0) }) == (100)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"switch_on_call(0) == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_on_call_1(1) }) == (200)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"switch_on_call(1) == 200".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_on_call_1(2) }) == (400)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"switch_on_call(2) == 400".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_on_call_1(99) }) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"switch_on_call(99) == -1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
