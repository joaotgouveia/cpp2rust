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
    let mut ops: [Option<unsafe fn(i32, i32) -> i32>; 3] = [Some(add_0), Some(sub_1), Some(mul_2)];
    (if ((unsafe { (ops[(0) as usize]).unwrap()(2, 3) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ops[0](2, 3) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { (ops[(1) as usize]).unwrap()(7, 4) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ops[1](7, 4) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { (ops[(2) as usize]).unwrap()(6, 5) }) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ops[2](6, 5) == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !((ops[(0) as usize]).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ops[0] != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((ops[(0) as usize]) == (Some(add_0))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ops[0] == add".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((ops[(0) as usize]) != (Some(sub_1))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ops[0] != sub".as_ptr(),
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
