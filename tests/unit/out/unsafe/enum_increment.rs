extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type color = u32;
pub const color_RED: color = 0;
pub const color_GREEN: color = 1;
pub const color_BLUE: color = 2;
pub const color_COLOR_LAST: color = 3;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut count: i32 = 0;
    let mut c: color = color_RED;
    'loop_: while ((((c as u32) < ((color_COLOR_LAST as i32) as u32)) as i32) != 0) {
        count.postfix_inc();
        c.postfix_inc();
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((count) == (3)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"count == 3".as_ptr().cast_mut()).cast_const(),
                        (c"enum_increment.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c: color = color_RED;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.postfix_inc() as u32) == ((color_RED as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c++ == RED".as_ptr().cast_mut()).cast_const(),
                        (c"enum_increment.c".as_ptr().cast_mut()).cast_const(),
                        13_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.prefix_inc() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"++c == BLUE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_increment.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.postfix_dec() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"c-- == BLUE".as_ptr().cast_mut()).cast_const(),
                        (c"enum_increment.c".as_ptr().cast_mut()).cast_const(),
                        15_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((c.prefix_dec() as u32) == ((color_RED as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"--c == RED".as_ptr().cast_mut()).cast_const(),
                        (c"enum_increment.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
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
