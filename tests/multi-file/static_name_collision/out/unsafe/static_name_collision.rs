extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut same_name_different_type_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 1 });
pub static mut same_name_same_type_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 5 });
pub unsafe fn a_foo_2() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_different_type_0));
}
pub unsafe fn a_bar_3() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_same_type_1));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { a_foo_2() }) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"a_foo() == 1".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        13_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { b_foo_5() }) == (1.0E+0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"b_foo() == 1.0f".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { a_bar_3() }) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"a_bar() == 5".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { b_bar_6() }) == (6)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"b_bar() == 6".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub static mut same_name_different_type_7: std::cell::LazyCell<f32> =
    std::cell::LazyCell::new(|| unsafe { 1.0E+0 });
pub static mut same_name_same_type_8: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 6 });
pub unsafe fn b_foo_5() -> f32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_different_type_7));
}
pub unsafe fn b_bar_6() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut same_name_same_type_8));
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const same_name_different_type_0);
    std::cell::LazyCell::force(&*&raw const same_name_same_type_1);
    std::cell::LazyCell::force(&*&raw const same_name_different_type_7);
    std::cell::LazyCell::force(&*&raw const same_name_same_type_8);
}
