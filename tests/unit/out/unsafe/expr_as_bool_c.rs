extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub unsafe fn cmp_eq_0(mut rc: i32) -> i32 {
    return (((rc) == (-1_i32)) as i32);
}
pub unsafe fn cmp_or_ptr_1(mut p: *const libc::c_char, mut q: *const libc::c_char) -> i32 {
    return (((!(p).is_null()) || (!(q).is_null())) as i32);
}
pub unsafe fn both_null_2(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> i32 {
    return ((((((s1).is_null()) as i32) != 0) && ((((s2).is_null()) as i32) != 0)) as i32);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0_i32;
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (((({
        b = a;
        b
    }) != (0)) as i32)
        != 0)
    {}
    if (a != 0) {}
    if ((((a) == (b)) as i32) != 0) {}
    if ((((a) < (b)) as i32) != 0) {}
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((a) == (b)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"a == b".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(({
                a = b;
                a
            }) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"!(a = b)".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        27_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut c: bool = false;
    c = ({
        a = b;
        a
    } != 0);
    c = (((({
        b = a;
        b
    }) != (0)) as i32)
        != 0);
    c = (a != 0);
    c = ((((a) == (b)) as i32) != 0);
    c = ((((a) < (b)) as i32) != 0);
    let mut x: i32 = 5;
    let mut y: i32 = 5;
    let mut eq: i32 = (((x) == (y)) as i32);
    let mut lt: i32 = (((x) < (y)) as i32);
    let mut neq: i32 = (((x) != (y)) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((eq) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"eq == 1".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        40_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((lt) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"lt == 0".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        41_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((neq) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"neq == 0".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut p1: *const libc::c_char = (c"hi".as_ptr().cast_mut()).cast_const();
    let mut p2: *const libc::c_char = std::ptr::null();
    let mut either: i32 = (((!(p1).is_null()) || (!(p2).is_null())) as i32);
    let mut both: i32 = (((!(p1).is_null()) && (!(p2).is_null())) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((either) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"either == 1".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        48_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((both) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"both == 0".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { cmp_eq_0(-1_i32) }) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"cmp_eq(-1) == 1".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        51_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { cmp_eq_0(0) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"cmp_eq(0) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        52_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { cmp_or_ptr_1(p1, p2) }) == (1)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"cmp_or_ptr(p1, p2) == 1".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        53_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { cmp_or_ptr_1(std::ptr::null(), std::ptr::null()) }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"cmp_or_ptr(NULL, NULL) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { both_null_2(std::ptr::null(), std::ptr::null()) }) == (1)) as i32) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"both_null(NULL, NULL) == 1".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        55_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { both_null_2(p1, std::ptr::null()) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"both_null(p1, NULL) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"expr_as_bool_c.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"int main()".as_ptr(),
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
