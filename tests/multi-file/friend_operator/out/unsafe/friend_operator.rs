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
pub struct S {
    pub a: i32,
    pub b: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s1: S = S { a: 1, b: 2 };
    let mut s2: S = S { a: 1, b: 2 };
    let mut s3: S = S { a: 1, b: 3 };
    (if (unsafe {
        let _x: *const S = &s1;
        operator_eq_0(_x, &s2)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s1 == s2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _x: *const S = &s1;
        operator_ne_2(_x, &s3)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s1 != s3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _x: *const S = &s1;
        operator_eq_0(_x, &s3)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"!(s1 == s3)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _x: *const S = &s1;
        operator_lt_3(_x, &s3)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s1 < s3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _x: *const S = &s3;
        operator_lt_3(_x, &s1)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"!(s3 < s1)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { compare_4(&s1, &s3) }) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"compare(s1, s3) == -1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { compare_4(&s3, &s1) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"compare(s3, s1) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { compare_4(&s1, &s2) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"compare(s1, s2) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn operator_eq_0(x: *const S, y: *const S) -> bool {
    return (((*x).a) == ((*y).a)) && (((*x).b) == ((*y).b));
}
pub unsafe fn operator_ne_2(x: *const S, y: *const S) -> bool {
    return !(unsafe {
        let _x: *const S = x;
        let _y: *const S = y;
        operator_eq_0(_x, _y)
    });
}
pub unsafe fn operator_lt_3(x: *const S, y: *const S) -> bool {
    return (((*x).a) < ((*y).a)) || ((((*x).a) == ((*y).a)) && (((*x).b) < ((*y).b)));
}
pub unsafe fn compare_4(x: *const S, y: *const S) -> i32 {
    if (unsafe {
        let _x: *const S = x;
        let _y: *const S = y;
        operator_lt_3(_x, _y)
    }) {
        return -1_i32;
    }
    if (unsafe {
        let _x: *const S = y;
        let _y: *const S = x;
        operator_lt_3(_x, _y)
    }) {
        return 1;
    }
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
