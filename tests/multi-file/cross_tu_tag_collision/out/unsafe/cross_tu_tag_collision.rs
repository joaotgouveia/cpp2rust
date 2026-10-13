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
pub struct widget {
    pub id: i32,
}
pub unsafe fn a_value_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 11;
    return w.id;
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
            if ((((unsafe { a_value_0() }) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"a_value() == 11".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { b_value_2() }) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_1(
                        (c"b_value() == 2".as_ptr().cast_mut()).cast_const(),
                        (c"a.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub type widget_enum = u32;
pub const widget_enum_WIDGET_A: widget_enum = 0;
pub const widget_enum_WIDGET_B: widget_enum = 1;
pub const widget_enum_WIDGET_C: widget_enum = 2;
pub unsafe fn b_value_2() -> i32 {
    let mut w: widget_enum = widget_enum_WIDGET_C;
    return (w as i32);
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
