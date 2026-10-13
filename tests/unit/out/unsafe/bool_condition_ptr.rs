extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut storage: i32 = 7;
    let mut p: *mut i32 = (&mut storage as *mut i32);
    let mut np: *mut i32 = std::ptr::null_mut();
    if !(p).is_null() {
        (if true {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"true".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    if !(!(p).is_null()) {
        (if false {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"false".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    if !(np).is_null() {
        (if false {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"false".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    if !(!(np).is_null()) {
        (if true {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"true".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    let mut iter: *mut i32 = p;
    let mut iters: i32 = 0;
    'loop_: while !(iter).is_null() {
        iters.prefix_inc();
        iter = std::ptr::null_mut();
    }
    (if ((iters) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"iters == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t3: i32 = if !(p).is_null() { 1 } else { 0 };
    (if ((t3) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t3 == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t4: i32 = if !(np).is_null() { 1 } else { 0 };
    (if ((t4) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t4 == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t5: i32 = (!(!(p).is_null()) as i32);
    (if ((t5) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t5 == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t6: i32 = (!(!(np).is_null()) as i32);
    (if ((t6) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t6 == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut b2: bool = !(p).is_null();
    let mut b3: bool = !(np).is_null();
    (if b2 {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"b2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(b3) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!b3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
