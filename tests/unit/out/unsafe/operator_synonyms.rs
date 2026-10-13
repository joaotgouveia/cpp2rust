extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: u32 = 12_u32;
    let mut b: u32 = 10_u32;
    let mut z: u32 = 0_u32;
    (if !(z != 0) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"not z".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (a != 0) && (b != 0) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a and b".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !((a != 0) && (z != 0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"not(a and z)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (a != 0) || (z != 0) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a or z".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((a) != (b)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a not_eq b".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((!a) == (!12_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(compl a) == compl 0b1100u".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((a) & (b)) == (8_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a bitand b) == 0b1000".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((a) | (b)) == (14_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a bitor b) == 0b1110".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((a) ^ (b)) == (6_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a xor b) == 0b0110".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    a &= b;
    (if ((a) == (8_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a == 0b1000".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    a |= b;
    (if ((a) == (10_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a == 0b1010".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    a ^= b;
    (if ((a) == (0_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
