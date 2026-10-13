extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut Static_int__s_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 55 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_0 {}
pub static mut Static_char__s_3: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { (55 as libc::c_char) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_2 {}
pub static mut Static_long__s_5: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { 55_i64 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_4 {}
pub static mut Range_int__lo_7: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut Range_int__hi_8: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 1 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_6 {}
pub static mut Range_long__lo_10: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { (-2_i32 as i64) });
pub static mut Range_long__hi_11: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { 2_i64 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_9 {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut Static_int__s_1)) = 22;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut Static_char__s_3)) = (33 as libc::c_char);
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Static_int__s_1)) == (22)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_12(
                c"Static<int>::s == 22".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut Static_char__s_3)) as i32) == (33)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_12(
                c"Static<char>::s == 33".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Static_long__s_5)) == (55_i64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_12(
                c"Static<long>::s == 55".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Range_int__lo_7)) == (-1_i32))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut Range_int__hi_8)) == (1))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_12(
                c"Range<int>::lo == -1 && Range<int>::hi == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut Range_long__lo_10)) == (-2_i32 as i64))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut Range_long__hi_11)) == (2_i64))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_12(
                c"Range<long>::lo == -2 && Range<long>::hi == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_12(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Static_int__s_1);
    std::cell::LazyCell::force(&*&raw const Static_char__s_3);
    std::cell::LazyCell::force(&*&raw const Static_long__s_5);
    std::cell::LazyCell::force(&*&raw const Range_int__lo_7);
    std::cell::LazyCell::force(&*&raw const Range_int__hi_8);
    std::cell::LazyCell::force(&*&raw const Range_long__lo_10);
    std::cell::LazyCell::force(&*&raw const Range_long__hi_11);
}
