extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: *mut i32) {
    (*x) *= 2;
}
pub unsafe fn maybe_call_1(mut cb: Option<unsafe fn(*mut i32)>, mut x: *mut i32) {
    if !(cb).is_none() {
        (unsafe { (cb).unwrap()(x) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 5;
    (unsafe { maybe_call_1(Some(double_it_0), (&mut a as *mut i32)) });
    (if ((a) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"a == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut b: i32 = 5;
    (unsafe { maybe_call_1(None, (&mut b as *mut i32)) });
    (if ((b) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"b == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut fn_: Option<unsafe fn(*mut i32)> = None;
    if !(!(fn_).is_none()) {
        fn_ = Some(double_it_0);
    }
    let mut c: i32 = 3;
    if !(fn_).is_none() {
        (unsafe { (fn_).unwrap()((&mut c as *mut i32)) });
    }
    (if ((c) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"c == 6".as_ptr(),
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
