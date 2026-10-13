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
    let mut arr: [libc::c_char; 7] = std::mem::transmute(*b"foobar\0");
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((arr[(0) as usize] as i32) == ('f' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"arr[0] == \'f\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
                        6_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((arr[(3) as usize] as i32) == ('b' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"arr[3] == \'b\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
                        7_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((arr[(5) as usize] as i32) == ('r' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"arr[5] == \'r\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
                        8_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((arr[(6) as usize] as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"arr[6] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut split_pieces: *const libc::c_char = (c"abcdefghi".as_ptr().cast_mut()).cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*split_pieces.offset((0) as isize)) as i32) == ('a' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"split_pieces[0] == \'a\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((*split_pieces.offset((3) as isize)) as i32) == ('d' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"split_pieces[3] == \'d\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((*split_pieces.offset((6) as isize)) as i32) == ('g' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"split_pieces[6] == \'g\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((*split_pieces.offset((8) as isize)) as i32) == ('i' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"split_pieces[8] == \'i\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*split_pieces.offset((9) as isize)) as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_0(
                        (c"split_pieces[9] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_concat_c.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"int main(void)".as_ptr(),
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
