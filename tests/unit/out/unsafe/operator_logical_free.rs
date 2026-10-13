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
    pub v: i32,
}
pub unsafe fn operator_not_0(a: *const S) -> bool {
    return (((*a).v) == (0));
}
pub unsafe fn operator_and_1(a: *const S, b: *const S) -> bool {
    return (((*a).v) != (0)) && (((*b).v) != (0));
}
pub unsafe fn operator_or_2(a: *const S, b: *const S) -> bool {
    return (((*a).v) != (0)) || (((*b).v) != (0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: S = S { v: 1 };
    let mut f: S = S { v: 0 };
    (if (unsafe {
        let _a: *const S = &f;
        operator_not_0(_a)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"!f".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _a: *const S = &t;
        operator_not_0(_a)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"!!t".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _a: *const S = &t;
        let _b: *const S = &t;
        operator_and_1(_a, _b)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"t && t".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _a: *const S = &t;
        operator_and_1(_a, &f)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"!(t && f)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _a: *const S = &t;
        operator_or_2(_a, &f)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"t || f".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _a: *const S = &f;
        let _b: *const S = &f;
        operator_or_2(_a, _b)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"!(f || f)".as_ptr(),
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
