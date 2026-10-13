extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pod {
    pub v: i32,
}
pub unsafe fn zero_0() -> *mut i32 {
    return std::ptr::null_mut();
}
pub unsafe fn zero_1() -> i64 {
    return 0_i64;
}
pub unsafe fn destroy_2(mut p: *mut i32) {}
pub unsafe fn destroy_3(mut p: *mut Pod) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i: i32 = 0_i32;
    let mut d: f64 = 0_f64;
    let mut p: *mut i32 = (unsafe { zero_0() });
    (if ((i) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"i == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((d) == (0.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"d == 0.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (p).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"p == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { zero_1() }) == (0_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"zero<long>() == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut x: i32 = 5;
    (unsafe { destroy_2((&mut x as *mut i32)) });
    (if ((x) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"x == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pod: Pod = Pod { v: 7 };
    (unsafe { destroy_3((&mut pod as *mut Pod)) });
    (if ((pod.v) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"pod.v == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
