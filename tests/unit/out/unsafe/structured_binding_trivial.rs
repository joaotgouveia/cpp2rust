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
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __decomp_0: Pair = Pair {
        first: 1,
        second: 2,
    };
    (if ((__decomp_0.first) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"in == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_0.second) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"fun == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: Pair = Pair {
        first: 10,
        second: 20,
    };
    let mut __decomp_2: Pair = p;
    __decomp_2.first = 11;
    __decomp_2.second += 1;
    (if ((__decomp_2.first) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"a == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_2.second) == (21)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"b == 21".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((p.first) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"p.first == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((p.second) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"p.second == 20".as_ptr(),
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
