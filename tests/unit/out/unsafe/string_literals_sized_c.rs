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
    let mut empty_buf: [libc::c_char; 256] = [0 as libc::c_char; 256];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((empty_buf[(0) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"empty_buf[0] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        5_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((empty_buf[(255) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"empty_buf[255] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        6_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut prefix_buf: [libc::c_char; 32] =
        std::mem::transmute(*b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((prefix_buf[(0) as usize] as i32) == ('%' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"prefix_buf[0] == \'%\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((prefix_buf[(1) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"prefix_buf[1] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((prefix_buf[(31) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"prefix_buf[31] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut short_buf: [libc::c_char; 16] = std::mem::transmute(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((short_buf[(0) as usize] as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"short_buf[0] == \'h\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
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
            if ((((short_buf[(1) as usize] as i32) == ('i' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"short_buf[1] == \'i\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
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
            if ((((short_buf[(2) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"short_buf[2] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        16_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((short_buf[(15) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"short_buf[15] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut exact_buf: [libc::c_char; 6] = std::mem::transmute(*b"hello\0");
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((exact_buf[(0) as usize] as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"exact_buf[0] == \'h\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((exact_buf[(4) as usize] as i32) == ('o' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"exact_buf[4] == \'o\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        21_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((exact_buf[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"exact_buf[5] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_sized_c.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
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
