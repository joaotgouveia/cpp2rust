extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_greeting_0() -> *const libc::c_char {
    return (c"hello".as_ptr().cast_mut()).cast_const();
}
pub unsafe fn get_empty_1() -> *const libc::c_char {
    return (c"".as_ptr().cast_mut()).cast_const();
}
pub unsafe fn get_branch_2(mut x: i32) -> *const libc::c_char {
    if ((((x) > (0)) as i32) != 0) {
        return (c"positive".as_ptr().cast_mut()).cast_const();
    }
    return (c"non-positive".as_ptr().cast_mut()).cast_const();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: *const libc::c_char = (unsafe { get_greeting_0() });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*a.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"a[0] == \'h\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((*a.offset((4) as isize)) as i32) == ('o' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"a[4] == \'o\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
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
            if (((((*a.offset((5) as isize)) as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"a[5] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        17_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut b: *const libc::c_char = (unsafe { get_empty_1() });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*b.offset((0) as isize)) as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"b[0] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c: *const libc::c_char = (unsafe { get_branch_2(1) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*c.offset((0) as isize)) as i32) == ('p' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"c[0] == \'p\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        23_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*c.offset((7) as isize)) as i32) == ('e' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"c[7] == \'e\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*c.offset((8) as isize)) as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"c[8] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        25_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut d: *const libc::c_char = (unsafe { get_branch_2(-1_i32) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*d.offset((0) as isize)) as i32) == ('n' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"d[0] == \'n\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        28_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*d.offset((11) as isize)) as i32) == ('e' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"d[11] == \'e\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        29_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*d.offset((12) as isize)) as i32) == ('\0' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"d[12] == \'\\0\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_literals_return_c.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
