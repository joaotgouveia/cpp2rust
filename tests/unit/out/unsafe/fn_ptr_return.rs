extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut x: i32) -> i32 {
    return ((x) + (1));
}
pub unsafe fn dec_1(mut x: i32) -> i32 {
    return ((x) - (1));
}
pub unsafe fn pick_2(mut choose_inc: i32) -> Option<unsafe fn(i32) -> i32> {
    if (choose_inc != 0) {
        return Some(inc_0);
    }
    return Some(dec_1);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Option<unsafe fn(i32) -> i32> = (unsafe { pick_2(1) });
    (if !((f).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"f != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((f) == (Some(inc_0))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"f == inc".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { (f).unwrap()(10) }) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"f(10) == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut g: Option<unsafe fn(i32) -> i32> = (unsafe { pick_2(0) });
    (if ((g) == (Some(dec_1))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"g == dec".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { (g).unwrap()(10) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"g(10) == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((f) != (g)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"f != g".as_ptr(),
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
