extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type E = u32;
pub const E_A: E = 2;
pub const E_B: E = 4;
pub const E_C: E = 2;
pub const E_D: E = 4;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((E_A as i32) == ((1) << (1))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"E::A == 1 << 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((E_B as i32) == ((1) << (2))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"E::B == 1 << 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((E_C as i32) == ((1) << (1))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"E::C == 1 << 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((E_D as i32) == ((1) << (2))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"E::D == 1 << 2".as_ptr(),
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
