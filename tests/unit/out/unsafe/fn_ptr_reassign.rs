extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_0(mut a: i32, mut b: i32) -> i32 {
    return ((a) + (b));
}
pub unsafe fn sub_1(mut a: i32, mut b: i32) -> i32 {
    return ((a) - (b));
}
pub unsafe fn mul_2(mut a: i32, mut b: i32) -> i32 {
    return ((a) * (b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(i32, i32) -> i32> = Some(add_0);
    (if ((unsafe { (fn_).unwrap()(3, 4) }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"fn(3, 4) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    fn_ = Some(sub_1);
    (if ((unsafe { (fn_).unwrap()(10, 3) }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"fn(10, 3) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    fn_ = Some(mul_2);
    (if ((unsafe { (fn_).unwrap()(6, 7) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"fn(6, 7) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    fn_ = None;
    (if (fn_).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"fn == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    fn_ = Some(add_0);
    (if !((fn_).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"fn != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { (fn_).unwrap()(1, 1) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"fn(1, 1) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
