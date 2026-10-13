extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_complex_cond_0(mut p: *mut i32, mut bias: i32) -> i32 {
    'switch: {
        match { ((*p) + (bias)) } {
            __v if __v == 0 => {
                return 1;
            }
            __v if __v == 5 => {
                return 2;
            }
            __v if __v == 10 => {
                return 3;
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
    let mut p_val: i32 = 5;
    (if ((unsafe { switch_complex_cond_0((&mut p_val as *mut i32), 0) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_complex_cond(&p_val, 0) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_complex_cond_0((&mut p_val as *mut i32), 5) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_complex_cond(&p_val, 5) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_complex_cond_0((&mut p_val as *mut i32), -5_i32) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_complex_cond(&p_val, -5) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { switch_complex_cond_0((&mut p_val as *mut i32), 99) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"switch_complex_cond(&p_val, 99) == 0".as_ptr(),
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
