extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_memcpy_0() {
    let src: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hello\0")));
    let dst: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
    let mut r: AnyPtr = ({
        memcpy_1(
            ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ((src.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            6_u64,
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (r).clone() } == { ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() }),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ((((*dst.borrow())[(0) as usize] as i32) == (('h' as i8) as i32))
                && (((*dst.borrow())[(1) as usize] as i32) == (('e' as i8) as i32)))
                && (((*dst.borrow())[(2) as usize] as i32) == (('l' as i8) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ((((*dst.borrow())[(3) as usize] as i32) == (('l' as i8) as i32))
                && (((*dst.borrow())[(4) as usize] as i32) == (('o' as i8) as i32)))
                && (((*dst.borrow())[(5) as usize] as i32) == (('\0' as i8) as i32)),
        )
    });
}
pub fn test_memset_3() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut r: AnyPtr = ({
        memset_4(
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            (('x' as i8) as i32),
            4_u64,
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (r).clone() } == { ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() }),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (((((*buf.borrow())[(0) as usize] as i32) == (('x' as i8) as i32))
                && (((*buf.borrow())[(1) as usize] as i32) == (('x' as i8) as i32)))
                && (((*buf.borrow())[(2) as usize] as i32) == (('x' as i8) as i32)))
                && (((*buf.borrow())[(3) as usize] as i32) == (('x' as i8) as i32)),
        )
    });
}
pub fn test_memcmp_5() {
    let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let b: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let c: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 9_i8, 4_i8])));
    ({
        cpp2rust_assert_fail_2(
            (({
                memcmp_6(
                    ((a.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    ((b.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    4_u64,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                memcmp_6(
                    ((a.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    ((c.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    4_u64,
                )
            }) < 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                memcmp_6(
                    ((c.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    ((a.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    4_u64,
                )
            }) > 0),
        )
    });
}
pub fn test_memmove_7() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('d' as i8),
        ('e' as i8),
        ('\0' as i8),
    ])));
    let mut r: AnyPtr = ({
        let ___dest: AnyPtr =
            ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any();
        let ___src: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
        memmove_8(___dest, ___src, 4_u64)
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (r).clone() } == {
                ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any()
            }),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ((((*buf.borrow())[(0) as usize] as i32) == (('a' as i8) as i32))
                && (((*buf.borrow())[(1) as usize] as i32) == (('a' as i8) as i32)))
                && (((*buf.borrow())[(2) as usize] as i32) == (('b' as i8) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ((((*buf.borrow())[(3) as usize] as i32) == (('c' as i8) as i32))
                && (((*buf.borrow())[(4) as usize] as i32) == (('d' as i8) as i32)))
                && (((*buf.borrow())[(5) as usize] as i32) == (('\0' as i8) as i32)),
        )
    });
}
pub fn test_strchr_9() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strchr_10((s).clone(), (('w' as i8) as i32)) });
    ({ cpp2rust_assert_fail_2(!((r).is_null())) });
    ({ cpp2rust_assert_fail_2((((r.read()) as i32) == (('w' as i8) as i32))) });
    ({ cpp2rust_assert_fail_2(({ strchr_10((s).clone(), (('z' as i8) as i32)) }).is_null()) });
}
pub fn test_strlen_11() {
    ({ cpp2rust_assert_fail_2((({ strlen_12(Ptr::<i8>::from_string_literal(b"")) }) == 0_u64)) });
    ({
        cpp2rust_assert_fail_2((({ strlen_12(Ptr::<i8>::from_string_literal(b"hello")) }) == 5_u64))
    });
    ({
        cpp2rust_assert_fail_2(
            (({ strlen_12(Ptr::<i8>::from_string_literal(b"hello world")) }) == 11_u64),
        )
    });
}
pub fn test_strcmp_13() {
    ({
        cpp2rust_assert_fail_2(
            (({
                strcmp_14(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"abc"),
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strcmp_14(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"abd"),
                )
            }) < 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strcmp_14(
                    Ptr::<i8>::from_string_literal(b"abd"),
                    Ptr::<i8>::from_string_literal(b"abc"),
                )
            }) > 0),
        )
    });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abc");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abd");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    ({
        cpp2rust_assert_fail_2(
            (({
                let ___s1: Ptr<i8> = (p).clone();
                let ___s2: Ptr<i8> = (p).clone();
                strcmp_14(___s1, ___s2)
            }) == 0),
        )
    });
    ({ cpp2rust_assert_fail_2((({ strcmp_14((p).clone(), (q).clone()) }) < 0)) });
    ({
        cpp2rust_assert_fail_2((({ strcmp_14((buf.as_pointer() as Ptr<i8>), (p).clone()) }) == 0))
    });
}
pub fn test_strncmp_15() {
    ({
        cpp2rust_assert_fail_2(
            (({
                strncmp_16(
                    Ptr::<i8>::from_string_literal(b"abcdef"),
                    Ptr::<i8>::from_string_literal(b"abcxyz"),
                    3_u64,
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strncmp_16(
                    Ptr::<i8>::from_string_literal(b"abcdef"),
                    Ptr::<i8>::from_string_literal(b"abcxyz"),
                    4_u64,
                )
            }) < 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strncmp_16(
                    Ptr::<i8>::from_string_literal(b"abcxyz"),
                    Ptr::<i8>::from_string_literal(b"abcdef"),
                    4_u64,
                )
            }) > 0),
        )
    });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcdef");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcxyz");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('d' as i8),
        ('e' as i8),
        ('f' as i8),
        ('\0' as i8),
    ])));
    let mut n: u64 = 3_u64;
    ({ cpp2rust_assert_fail_2((({ strncmp_16((p).clone(), (q).clone(), n) }) == 0)) });
    ({
        cpp2rust_assert_fail_2(
            (({ strncmp_16((p).clone(), (q).clone(), (n).wrapping_add(1_u64)) }) < 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({ strncmp_16((buf.as_pointer() as Ptr<i8>), (p).clone(), 6_u64) }) == 0),
        )
    });
}
pub fn test_memchr_17() {
    let data: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([16_i8, 32_i8, 48_i8, 64_i8])));
    let mut r: AnyPtr = ({
        memchr_18(
            ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            48,
            4_u64,
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ({ (r).clone() } == {
                (((data.as_pointer() as Ptr<i8>).offset(2)) as Ptr<i8>).to_any()
            }),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            ({
                memchr_18(
                    ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    153,
                    4_u64,
                )
            })
            .is_null(),
        )
    });
    let mut p: AnyPtr = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut n: u64 = 4_u64;
    ({ cpp2rust_assert_fail_2(({ ({ memchr_18((p).clone(), 16, n) }) } == { (p).clone() })) });
}
pub fn test_strrchr_19() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strrchr_20((s).clone(), (('l' as i8) as i32)) });
    ({ cpp2rust_assert_fail_2(!((r).is_null())) });
    ({ cpp2rust_assert_fail_2((((r.read()) as i32) == (('l' as i8) as i32))) });
    ({ cpp2rust_assert_fail_2(({ (r).clone() } == { s.offset((9) as isize) })) });
    ({ cpp2rust_assert_fail_2(({ strrchr_20((s).clone(), (('z' as i8) as i32)) }).is_null()) });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('a' as i8),
        ('\0' as i8),
    ])));
    ({
        cpp2rust_assert_fail_2(
            (({ strrchr_21((buf.as_pointer() as Ptr<i8>), (('a' as i8) as i32)) })
                == ((buf.as_pointer() as Ptr<i8>).offset(2))),
        )
    });
}
pub fn test_strdup_22() {
    let mut d: Ptr<i8> = ({ strdup_23(Ptr::<i8>::from_string_literal(b"hello")) });
    ({ cpp2rust_assert_fail_2(!((d).is_null())) });
    ({
        cpp2rust_assert_fail_2(
            (({ strcmp_14((d).clone(), Ptr::<i8>::from_string_literal(b"hello")) }) == 0),
        )
    });
    ({ free_24((d).to_any()) });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"world");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    let mut d2: Ptr<i8> = ({ strdup_23((p).clone()) });
    ({ cpp2rust_assert_fail_2(!((d2).is_null())) });
    ({ cpp2rust_assert_fail_2((({ strcmp_14((d2).clone(), (p).clone()) }) == 0)) });
    ({ free_24((d2).to_any()) });
    let mut d3: Ptr<i8> = ({ strdup_23((buf.as_pointer() as Ptr<i8>)) });
    ({ cpp2rust_assert_fail_2(!((d3).is_null())) });
    ({
        cpp2rust_assert_fail_2((({ strcmp_14((d3).clone(), (buf.as_pointer() as Ptr<i8>)) }) == 0))
    });
    ({ free_24((d3).to_any()) });
}
pub fn test_strcspn_25() {
    ({
        cpp2rust_assert_fail_2(
            (({
                strcspn_26(
                    Ptr::<i8>::from_string_literal(b"hello"),
                    Ptr::<i8>::from_string_literal(b"el"),
                )
            }) == 1_u64),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strcspn_26(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"xyz"),
                )
            }) == 3_u64),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strcspn_26(
                    Ptr::<i8>::from_string_literal(b""),
                    Ptr::<i8>::from_string_literal(b"abc"),
                )
            }) == 0_u64),
        )
    });
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut rej: Ptr<i8> = Ptr::<i8>::from_string_literal(b"el");
    ({ cpp2rust_assert_fail_2((({ strcspn_26((s).clone(), (rej).clone()) }) == 1_u64)) });
}
pub fn test_strspn_27() {
    ({
        cpp2rust_assert_fail_2(
            (({
                strspn_28(
                    Ptr::<i8>::from_string_literal(b"hello"),
                    Ptr::<i8>::from_string_literal(b"hel"),
                )
            }) == 4_u64),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strspn_28(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"xyz"),
                )
            }) == 0_u64),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strspn_28(
                    Ptr::<i8>::from_string_literal(b"aaa"),
                    Ptr::<i8>::from_string_literal(b"a"),
                )
            }) == 3_u64),
        )
    });
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut acc: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hel");
    ({ cpp2rust_assert_fail_2((({ strspn_28((s).clone(), (acc).clone()) }) == 4_u64)) });
}
pub fn test_strstr_29() {
    let mut h: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strstr_30((h).clone(), Ptr::<i8>::from_string_literal(b"world")) });
    ({ cpp2rust_assert_fail_2(!((r).is_null())) });
    ({ cpp2rust_assert_fail_2(({ (r).clone() } == { h.offset((6) as isize) })) });
    ({
        cpp2rust_assert_fail_2(
            ({ strstr_30((h).clone(), Ptr::<i8>::from_string_literal(b"xyz")) }).is_null(),
        )
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('h' as i8),
        ('e' as i8),
        ('l' as i8),
        ('l' as i8),
        ('o' as i8),
        ('\0' as i8),
    ])));
    ({
        cpp2rust_assert_fail_2(
            (({
                strstr_31(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"ll"),
                )
            }) == ((buf.as_pointer() as Ptr<i8>).offset(2))),
        )
    });
}
pub fn test_strpbrk_32() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strpbrk_33((s).clone(), Ptr::<i8>::from_string_literal(b"wo")) });
    ({ cpp2rust_assert_fail_2(!((r).is_null())) });
    ({ cpp2rust_assert_fail_2(({ (r).clone() } == { s.offset((4) as isize) })) });
    ({
        cpp2rust_assert_fail_2(
            ({ strpbrk_33((s).clone(), Ptr::<i8>::from_string_literal(b"xyz")) }).is_null(),
        )
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    ({
        cpp2rust_assert_fail_2(
            (({
                strpbrk_34(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"b"),
                )
            }) == ((buf.as_pointer() as Ptr<i8>).offset(1))),
        )
    });
}
pub fn test_strcasecmp_35() {
    ({
        cpp2rust_assert_fail_2(
            (({
                strcasecmp_36(
                    Ptr::<i8>::from_string_literal(b"HELLO"),
                    Ptr::<i8>::from_string_literal(b"hello"),
                )
            }) == 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strcasecmp_36(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"abd"),
                )
            }) < 0),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (({
                strcasecmp_36(
                    Ptr::<i8>::from_string_literal(b"abd"),
                    Ptr::<i8>::from_string_literal(b"abc"),
                )
            }) > 0),
        )
    });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"FOO");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
    ({ cpp2rust_assert_fail_2((({ strcasecmp_36((p).clone(), (q).clone()) }) == 0)) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_memcpy_0() });
    ({ test_memset_3() });
    ({ test_memcmp_5() });
    ({ test_memmove_7() });
    ({ test_strchr_9() });
    ({ test_strlen_11() });
    ({ test_strcmp_13() });
    ({ test_strncmp_15() });
    ({ test_memchr_17() });
    ({ test_strrchr_19() });
    ({ test_strdup_22() });
    ({ test_strcspn_25() });
    ({ test_strspn_27() });
    ({ test_strstr_29() });
    ({ test_strpbrk_32() });
    ({ test_strcasecmp_35() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
