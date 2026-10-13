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
pub struct Triple {
    pub a: i32,
    pub b: bool,
    pub c: i32,
}
pub unsafe fn sum_0(t: *const Triple) -> i32 {
    let __decomp_1: *const Triple = t;
    return ((((*__decomp_1).a) + (if (*__decomp_1).b { 1 } else { 0 })) + ((*__decomp_1).c));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: Triple = Triple {
        a: 10,
        b: false,
        c: 20,
    };
    let __decomp_2: *mut Triple = &mut t;
    (*__decomp_2).a = 11;
    (*__decomp_2).b = true;
    (*__decomp_2).c += 1;
    (if ((t.a) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"t.a == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if t.b {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"t.b".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((t.c) == (21)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"t.c == 21".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    t.a = 12;
    (if (((*__decomp_2).a) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"pos == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let __decomp_4: *const Triple = &t;
    t.c = 30;
    (if (((*__decomp_4).a) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ca == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (*__decomp_4).b {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"cb".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*__decomp_4).c) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"cc == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { sum_0(&t) }) == (43)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"sum(t) == 43".as_ptr(),
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
