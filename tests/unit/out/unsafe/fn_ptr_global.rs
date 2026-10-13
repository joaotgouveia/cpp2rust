extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn triple_it_1(mut x: i32) -> i32 {
    return ((x) * (3));
}
pub static mut g_op_2: std::cell::LazyCell<Option<unsafe fn(i32) -> i32>> =
    std::cell::LazyCell::new(|| unsafe { None });
pub unsafe fn set_op_3(mut fn_: Option<unsafe fn(i32) -> i32>) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)) = fn_;
}
pub unsafe fn call_op_4(mut x: i32) -> i32 {
    if !(*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)).is_none() {
        return (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)).unwrap()(x) });
    }
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { call_op_4(5) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_op(5) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { set_op_3(Some(double_it_0)) });
    (if !((*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"g_op != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)) == (Some(double_it_0))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"g_op == double_it".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_op_4(5) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_op(5) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { set_op_3(Some(triple_it_1)) });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)) == (Some(triple_it_1))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"g_op == triple_it".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_op_4(5) }) == (15)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_op(5) == 15".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { set_op_3(None) });
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut g_op_2)).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"g_op == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_op_4(5) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_op(5) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_5(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_op_2);
}
