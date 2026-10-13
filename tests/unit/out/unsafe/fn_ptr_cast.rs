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
pub unsafe fn test_roundtrip_1() {
    let mut fn_: Option<unsafe fn(i32) -> i32> = Some(double_it_0);
    (if ((unsafe { (fn_).unwrap()(5) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"fn(5) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_roundtrip()".as_ptr(),
            )
        });
    });
    let mut gfn: Option<unsafe fn()> =
        std::mem::transmute::<Option<unsafe fn(i32) -> i32>, Option<unsafe fn()>>(fn_);
    (if !((gfn).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"gfn != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_roundtrip()".as_ptr(),
            )
        });
    });
    let mut fn2: Option<unsafe fn(i32) -> i32> =
        std::mem::transmute::<Option<unsafe fn()>, Option<unsafe fn(i32) -> i32>>(gfn);
    (if ((unsafe { (fn2).unwrap()(5) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"fn2(5) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_roundtrip()".as_ptr(),
            )
        });
    });
    (if ((fn2) == (fn_)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"fn2 == fn".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_roundtrip()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_double_cast_3() {
    let mut fn_: Option<unsafe fn(i32) -> i32> = Some(double_it_0);
    let mut fn2: Option<unsafe fn(i32) -> i32> =
        std::mem::transmute::<Option<unsafe fn()>, Option<unsafe fn(i32) -> i32>>(
            std::mem::transmute::<Option<unsafe fn(i32) -> i32>, Option<unsafe fn()>>(fn_),
        );
    (if ((unsafe { (fn2).unwrap()(5) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"fn2(5) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_double_cast()".as_ptr(),
            )
        });
    });
    (if ((fn2) == (fn_)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"fn2 == fn".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_double_cast()".as_ptr(),
            )
        });
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Command {
    pub data: *mut ::libc::c_void,
}
pub unsafe fn test_void_ptr_to_fn_4() {
    let mut cmd: Command = <Command>::default();
    cmd.data = std::mem::transmute::<Option<unsafe fn(i32) -> i32>, *mut ::libc::c_void>(Some(
        double_it_0,
    ));
    let mut fn_: Option<unsafe fn(i32) -> i32> =
        std::mem::transmute::<*mut ::libc::c_void, Option<unsafe fn(i32) -> i32>>(cmd.data);
    (if ((unsafe { (fn_).unwrap()(5) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"fn(5) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_void_ptr_to_fn()".as_ptr(),
            )
        });
    });
}
pub unsafe fn add_offset_5(mut base: *mut i32, mut offset: i32) -> i32 {
    return ((*base) + (offset));
}
pub unsafe fn test_call_through_cast_6() {
    let mut gfn: Option<unsafe fn(*mut ::libc::c_void, i32) -> i32> = std::mem::transmute::<
        Option<unsafe fn(*mut i32, i32) -> i32>,
        Option<unsafe fn(*mut ::libc::c_void, i32) -> i32>,
    >(Some(add_offset_5));
    let mut val: i32 = 100;
    let mut result: i32 =
        (unsafe { (gfn).unwrap()(((&mut val as *mut i32) as *mut ::libc::c_void), 42) });
    (if ((result) == (142)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"result == 142".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_call_through_cast()".as_ptr(),
            )
        });
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_roundtrip_1() });
    (unsafe { test_double_cast_3() });
    (unsafe { test_void_ptr_to_fn_4() });
    (unsafe { test_call_through_cast_6() });
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
