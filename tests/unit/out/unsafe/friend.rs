extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(v: *const V) -> i32 {
    return (*v).x;
}
pub unsafe fn operator_eq_1(a: *const V, b: *const V) -> bool {
    return (((*a).x) == ((*b).x));
}
pub unsafe fn scaled_2(v: *const V, mut k: i32) -> i32 {
    return (((*v).x) * (k));
}
pub unsafe fn scaled_3(v: *const V, mut k: f64) -> f64 {
    return (((*v).x as f64) * (k));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct V {
    pub x: i32,
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const V, other as *const V) }
    }
}
impl std::cmp::Eq for V {}
pub unsafe fn get_5(w: *const W_4) -> i32 {
    return (*w).x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct W_4 {
    pub x: i32,
}
pub unsafe fn get_7(w: *const W_6) -> i64 {
    return (*w).x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct W_6 {
    pub x: i64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {
    pub x: i32,
}
pub unsafe fn declared_then_defined_8(d: *const D) -> i32 {
    return (((*d).x) + (1));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: V = V { x: 3 };
    let mut b: V = V { x: 3 };
    let mut c: V = V { x: 4 };
    (if ((unsafe { get_0(&a) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"get(a) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _a: *const V = &a;
        operator_eq_1(_a, &b)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"a == b".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _a: *const V = &a;
        operator_eq_1(_a, &c)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"!(a == c)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { scaled_2(&c, 2) }) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"scaled(c, 2) == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { scaled_3(&c, 1.5E+0) }) == (6.0E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"scaled(c, 1.5) == 6.0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut wi: W_4 = W_4 { x: 5 };
    let mut wl: W_6 = W_6 { x: 6_i64 };
    (if ((unsafe { get_5(&wi) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"get(wi) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { get_7(&wl) }) == (6_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"get(wl) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut d: D = D { x: 7 };
    (if ((unsafe { declared_then_defined_8(&d) }) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"declared_then_defined(d) == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_9(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
