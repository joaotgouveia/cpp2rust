extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = u32;
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
pub unsafe fn switch_enum_0(mut c: Color) -> i32 {
    'switch: {
        match { (c as i32) } {
            __v if __v == (Color_kRed as i32) => {
                return 10;
            }
            __v if __v == (Color_kGreen as i32) => {
                return 20;
            }
            __v if __v == (Color_kBlue as i32) => {
                return 30;
            }
            _ => {}
        }
    };
    return -1_i32;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { switch_enum_0(Color_kRed) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_enum(kRed) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_enum_0(Color_kGreen) }) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_enum(kGreen) == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_enum_0(Color_kBlue) }) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_enum(kBlue) == 30".as_ptr(),
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
