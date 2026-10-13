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
    let mut x1: i32 = -1_i32;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((x1) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"x1 == -1".as_ptr().cast_mut()).cast_const(),
                        (c"signed_integer.c".as_ptr().cast_mut()).cast_const(),
                        5_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut x2: i8 = -1_i8;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((x2 as i32) == (-1_i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"x2 == -1".as_ptr().cast_mut()).cast_const(),
                        (c"signed_integer.c".as_ptr().cast_mut()).cast_const(),
                        7_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut u1: u32 = 5_u32;
    let mut u2: u32 = (u1).wrapping_neg();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((u2) == (4294967291_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"u2 == 4294967291u".as_ptr().cast_mut()).cast_const(),
                        (c"signed_integer.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c1: libc::c_char = (-1 as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((c1 as u8) as i32) == (255)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"(unsigned char)c1 == 0xFF".as_ptr().cast_mut()).cast_const(),
                        (c"signed_integer.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
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
