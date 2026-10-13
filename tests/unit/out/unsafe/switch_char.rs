extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_char_0(mut c: libc::c_char) -> i32 {
    'switch: {
        match { (c as i32) } {
            __v if __v == (('a' as libc::c_char) as i32) => {
                return 1;
            }
            __v if __v == (('b' as libc::c_char) as i32) => {
                return 2;
            }
            __v if __v == (('\n' as libc::c_char) as i32) => {
                return 3;
            }
            __v if __v == (('\0' as libc::c_char) as i32) => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub type Color = u32;
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { switch_char_0(('a' as libc::c_char)) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_char(\'a\') == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_char_0(('b' as libc::c_char)) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_char(\'b\') == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_char_0(('\n' as libc::c_char)) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_char(\'\\n\') == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_char_0(('\0' as libc::c_char)) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_char(\'\\0\') == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_char_0(('z' as libc::c_char)) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_char(\'z\') == 0".as_ptr(),
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
