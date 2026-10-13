extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub unsafe fn test_expect_0() {
    let mut x: i32 = 42;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((unsafe { __builtin_expect_1(((((x) == (42)) as i32) as i64), 1_i64) }) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"__builtin_expect(x == 42, 1)".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"void test_expect(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_ctz_3() {
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
}
pub unsafe fn test_clz_4() {
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
}
pub unsafe fn test_bswap16_5() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((unsafe { __bswap_16_6((4660 as u16)) }) as i32) == (13330)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"bswap_16((unsigned short)0x1234) == 0x3412"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"void test_bswap16(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_bswap32_7() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { __bswap_32_8(305419896_u32) }) == (2018915346_u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"bswap_32(0x12345678U) == 0x78563412U".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        32_u32,
                        c"void test_bswap32(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_bswap64_9() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { __bswap_64_10(81985529216486895_u64) }) == (17279655951921914625_u64))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"bswap_64(0x0123456789ABCDEFULL) == 0xEFCDAB8967452301ULL"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        40_u32,
                        c"void test_bswap64(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_ctzl_11() {
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
}
pub unsafe fn test_popcountl_12() {
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
}
pub unsafe fn test_clzl_13() {
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
}
pub unsafe fn test_inff_14() {
    let mut inf: f32 = (unsafe { __builtin_inff_15() });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((inf) > (0.0E+0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"inf > 0.0f".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        63_u32,
                        c"void test_inff(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((inf) == ((inf) * (2.0E+0))) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"inf == inf * 2.0f".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        64_u32,
                        c"void test_inff(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((1.0E+0) / (inf)) == (0.0E+0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"1.0f / inf == 0.0f".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        65_u32,
                        c"void test_inff(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_nanf_16() {
    let mut nan: f32 = (unsafe { __builtin_nanf_17((c"".as_ptr().cast_mut()).cast_const()) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((nan) != (nan)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"nan != nan".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        70_u32,
                        c"void test_nanf(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if !((((nan) < (0.0E+0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"!(nan < 0.0f)".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        71_u32,
                        c"void test_nanf(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if !((((nan) > (0.0E+0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"!(nan > 0.0f)".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        72_u32,
                        c"void test_nanf(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_mul_overflow_long_18() {
    let mut r: i64 = 0_i64;
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(unsafe { __builtin_mul_overflow_19() }) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"!__builtin_mul_overflow(3L, 7L, &r)".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        77_u32,
                        c"void test_mul_overflow_long(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (21_i64)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == 21".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        78_u32,
                        c"void test_mul_overflow_long(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (unsafe { __builtin_mul_overflow_19() }) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"__builtin_mul_overflow(LONG_MAX, 2L, &r)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        79_u32,
                        c"void test_mul_overflow_long(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_mul_overflow_long_long_20() {
    let mut r: i64 = 0_i64;
    {
        &(::std::mem::size_of::<i32>());
        {
            if !(unsafe { __builtin_mul_overflow_19() }) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"!__builtin_mul_overflow(1000LL, 1000LL, &r)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        84_u32,
                        c"void test_mul_overflow_long_long(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (1000000_i64)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == 1000000".as_ptr().cast_mut()).cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        85_u32,
                        c"void test_mul_overflow_long_long(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (unsafe { __builtin_mul_overflow_19() }) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"__builtin_mul_overflow(LLONG_MAX, 2LL, &r)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"builtin.c".as_ptr().cast_mut()).cast_const(),
                        86_u32,
                        c"void test_mul_overflow_long_long(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_expect_0() });
    (unsafe { test_ctz_3() });
    (unsafe { test_clz_4() });
    (unsafe { test_bswap16_5() });
    (unsafe { test_bswap32_7() });
    (unsafe { test_bswap64_9() });
    (unsafe { test_ctzl_11() });
    (unsafe { test_popcountl_12() });
    (unsafe { test_clzl_13() });
    (unsafe { test_inff_14() });
    (unsafe { test_nanf_16() });
    (unsafe { test_mul_overflow_long_18() });
    (unsafe { test_mul_overflow_long_long_20() });
    return 0;
}
pub unsafe fn __builtin_expect_1(mut _a0: i64, mut _a1: i64) -> i64 {
    unimplemented!()
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __bswap_16_6(mut __bsx: u16) -> u16 {
    unimplemented!()
}
pub unsafe fn __bswap_32_8(mut __bsx: u32) -> u32 {
    unimplemented!()
}
pub unsafe fn __bswap_64_10(mut __bsx: u64) -> u64 {
    unimplemented!()
}
pub unsafe fn __builtin_inff_15() -> f32 {
    unimplemented!()
}
pub unsafe fn __builtin_nanf_17(mut _a0: *const libc::c_char) -> f32 {
    unimplemented!()
}
pub unsafe fn __builtin_mul_overflow_19() -> bool {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
