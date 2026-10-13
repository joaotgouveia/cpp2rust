extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
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
pub fn test_expect_0() {
    let mut x: i32 = 42;
    {
        &(4usize);
        {
            if (({ __builtin_expect_1((((x == 42) as i32) as i64), 1_i64) }) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"__builtin_expect(x == 42, 1)"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"void test_expect(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_ctz_3() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
}
pub fn test_clz_4() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
}
pub fn test_bswap16_5() {
    {
        &(4usize);
        {
            if ((((({ __bswap_16_6((4660 as u16)) }) as i32) == 13330) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"bswap_16((unsigned short)0x1234) == 0x3412",
                        ),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        24_u32,
                        Ptr::<i8>::from_string_literal(b"void test_bswap16(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_bswap32_7() {
    {
        &(4usize);
        {
            if (((({ __bswap_32_8(305419896_u32) }) == 2018915346_u32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"bswap_32(0x12345678U) == 0x78563412U"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        32_u32,
                        Ptr::<i8>::from_string_literal(b"void test_bswap32(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_bswap64_9() {
    {
        &(4usize);
        {
            if (((({ __bswap_64_10(81985529216486895_u64) }) == 17279655951921914625_u64) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"bswap_64(0x0123456789ABCDEFULL) == 0xEFCDAB8967452301ULL",
                        ),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        40_u32,
                        Ptr::<i8>::from_string_literal(b"void test_bswap64(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_ctzl_11() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
}
pub fn test_popcountl_12() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
}
pub fn test_clzl_13() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
}
pub fn test_inff_14() {
    let mut inf: f32 = ({ __builtin_inff_15() });
    {
        &(4usize);
        {
            if (((inf > 0.0E+0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"inf > 0.0f"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        63_u32,
                        Ptr::<i8>::from_string_literal(b"void test_inff(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((inf == (inf * 2.0E+0)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"inf == inf * 2.0f"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        64_u32,
                        Ptr::<i8>::from_string_literal(b"void test_inff(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((1.0E+0 / inf) == 0.0E+0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"1.0f / inf == 0.0f"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        65_u32,
                        Ptr::<i8>::from_string_literal(b"void test_inff(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_nanf_16() {
    let mut nan: f32 = ({ __builtin_nanf_17(Ptr::<i8>::from_string_literal(b"")) });
    {
        &(4usize);
        {
            if (((nan != nan) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"nan != nan"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        70_u32,
                        Ptr::<i8>::from_string_literal(b"void test_nanf(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if !(((nan < 0.0E+0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"!(nan < 0.0f)"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        71_u32,
                        Ptr::<i8>::from_string_literal(b"void test_nanf(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if !(((nan > 0.0E+0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"!(nan > 0.0f)"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        72_u32,
                        Ptr::<i8>::from_string_literal(b"void test_nanf(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_mul_overflow_long_18() {
    let r: Value<i64> = Rc::new(RefCell::new(0_i64));
    {
        &(4usize);
        {
            if !({ __builtin_mul_overflow_19() }) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"!__builtin_mul_overflow(3L, 7L, &r)"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        77_u32,
                        Ptr::<i8>::from_string_literal(b"void test_mul_overflow_long(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*r.borrow()) == 21_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == 21"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        78_u32,
                        Ptr::<i8>::from_string_literal(b"void test_mul_overflow_long(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ({ __builtin_mul_overflow_19() }) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"__builtin_mul_overflow(LONG_MAX, 2L, &r)"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        79_u32,
                        Ptr::<i8>::from_string_literal(b"void test_mul_overflow_long(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_mul_overflow_long_long_20() {
    let r: Value<i64> = Rc::new(RefCell::new(0_i64));
    {
        &(4usize);
        {
            if !({ __builtin_mul_overflow_19() }) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"!__builtin_mul_overflow(1000LL, 1000LL, &r)",
                        ),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        84_u32,
                        Ptr::<i8>::from_string_literal(b"void test_mul_overflow_long_long(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*r.borrow()) == 1000000_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == 1000000"),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        85_u32,
                        Ptr::<i8>::from_string_literal(b"void test_mul_overflow_long_long(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ({ __builtin_mul_overflow_19() }) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"__builtin_mul_overflow(LLONG_MAX, 2LL, &r)",
                        ),
                        Ptr::<i8>::from_string_literal(b"builtin.c"),
                        86_u32,
                        Ptr::<i8>::from_string_literal(b"void test_mul_overflow_long_long(void)"),
                    )
                });
            }
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_expect_0() });
    ({ test_ctz_3() });
    ({ test_clz_4() });
    ({ test_bswap16_5() });
    ({ test_bswap32_7() });
    ({ test_bswap64_9() });
    ({ test_ctzl_11() });
    ({ test_popcountl_12() });
    ({ test_clzl_13() });
    ({ test_inff_14() });
    ({ test_nanf_16() });
    ({ test_mul_overflow_long_18() });
    ({ test_mul_overflow_long_long_20() });
    return 0;
}
pub fn __builtin_expect_1(mut _a0: i64, mut _a1: i64) -> i64 {
    unimplemented!()
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __bswap_16_6(mut __bsx: u16) -> u16 {
    unimplemented!()
}
pub fn __bswap_32_8(mut __bsx: u32) -> u32 {
    unimplemented!()
}
pub fn __bswap_64_10(mut __bsx: u64) -> u64 {
    unimplemented!()
}
pub fn __builtin_inff_15() -> f32 {
    unimplemented!()
}
pub fn __builtin_nanf_17(mut _a0: Ptr<i8>) -> f32 {
    unimplemented!()
}
pub fn __builtin_mul_overflow_19() -> bool {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
