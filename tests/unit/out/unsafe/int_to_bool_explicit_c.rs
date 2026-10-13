extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut flag: u32 = 7_u32;
    let mut b1: bool = (flag != 0);
    let mut b2: bool = (0_u32 != 0);
    {
        &(::std::mem::size_of::<i32>());
        {
            if b1 {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"b1".as_ptr().cast_mut()).cast_const(),
                        (c"int_to_bool_explicit_c.c".as_ptr().cast_mut()).cast_const(),
                        8_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(b2) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"!b2".as_ptr().cast_mut()).cast_const(),
                        (c"int_to_bool_explicit_c.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
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
