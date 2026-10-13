extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn a_0() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut i_1));
}
static mut i_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn b_2() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut i_3));
}
static mut i_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 2 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { a_0() }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"a() == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { b_2() }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"b() == 2".as_ptr(),
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
