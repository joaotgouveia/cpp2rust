extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn test_memcpy_0() {
    let src: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hello\0")));
    let dst: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
    let mut r: AnyPtr = ({
        memcpy_1(
            ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ((src.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            6_usize,
        )
    });
    {
        &(4usize);
        {
            if ((({ (r).clone() } == { ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() })
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == dst"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memcpy(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((((((((*dst.borrow())[(0) as usize] as i32) == ('h' as i32)) as i32) != 0)
                && (((((*dst.borrow())[(1) as usize] as i32) == ('e' as i32)) as i32) != 0))
                as i32)
                != 0)
                && (((((*dst.borrow())[(2) as usize] as i32) == ('l' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"dst[0] == \'h\' && dst[1] == \'e\' && dst[2] == \'l\'",
                        ),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memcpy(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((((((((*dst.borrow())[(3) as usize] as i32) == ('l' as i32)) as i32) != 0)
                && (((((*dst.borrow())[(4) as usize] as i32) == ('o' as i32)) as i32) != 0))
                as i32)
                != 0)
                && (((((*dst.borrow())[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"dst[3] == \'l\' && dst[4] == \'o\' && dst[5] == \'\\0\'",
                        ),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memcpy(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_memset_3() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut r: AnyPtr = ({
        memset_4(
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ('x' as i32),
            4_usize,
        )
    });
    {
        &(4usize);
        {
            if ((({ (r).clone() } == { ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() })
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == buf"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memset(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((((((((((((*buf.borrow())[(0) as usize] as i32) == ('x' as i32)) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == ('x' as i32)) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(2) as usize] as i32) == ('x' as i32)) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(3) as usize] as i32) == ('x' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2 ( Ptr::<i8 >::from_string_literal(b"buf[0] == \'x\' && buf[1] == \'x\' && buf[2] == \'x\' && buf[3] == \'x\'" )  , Ptr::<i8 >::from_string_literal(b"string_h.c" )  , 19_u32  , Ptr::<i8 >::from_string_literal(b"void test_memset(void)" )  , )
                });
            }
        }
    };
}
pub fn test_memcmp_5() {
    let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let b: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let c: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 9_i8, 4_i8])));
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
}
pub fn test_memmove_6() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('c' as i32) as i8),
        (('d' as i32) as i8),
        (('e' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    let mut r: AnyPtr = ({
        let ___dest: AnyPtr =
            ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any();
        let ___src: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
        memmove_7(___dest, ___src, 4_usize)
    });
    {
        &(4usize);
        {
            if ((({ (r).clone() } == {
                ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any()
            }) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == buf + 1"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        34_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memmove(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((((((((*buf.borrow())[(0) as usize] as i32) == ('a' as i32)) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == ('a' as i32)) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(2) as usize] as i32) == ('b' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"buf[0] == \'a\' && buf[1] == \'a\' && buf[2] == \'b\'",
                        ),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memmove(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((((((((*buf.borrow())[(3) as usize] as i32) == ('c' as i32)) as i32) != 0)
                && (((((*buf.borrow())[(4) as usize] as i32) == ('d' as i32)) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"buf[3] == \'c\' && buf[4] == \'d\' && buf[5] == \'\\0\'",
                        ),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        36_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memmove(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strchr_8() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strchr_9(s.reinterpret_cast::<i8>(), ('w' as i32)) });
    {
        &(4usize);
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r != NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        42_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strchr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((r.read()) as i32) == ('w' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"*r == \'w\'"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        43_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strchr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strchr_9(s.reinterpret_cast::<i8>(), ('z' as i32)) }).is_null()) as i32) != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strchr((char *)s, \'z\') == NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        44_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strchr(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strlen_10() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"one\0two\0")));
    let mut first: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
    let mut second: Ptr<i8> = ((buf.as_pointer() as Ptr<i8>)
        .offset(({ strlen_11((first).clone()) }).wrapping_add(1_usize)));
    {
        &(4usize);
        {
            if (((({ strcmp_12((second).clone(), Ptr::<i8>::from_string_literal(b"two")) }) == 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(second, \"two\") == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        54_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strlen(void)"),
                    )
                });
            }
        }
    };
    let big : Value<Box<[ i8  ]> > = Rc::new(RefCell::new(i8::array_from_literal(b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") )) ;
    {
        &(4usize);
        {
            if (((({ strlen_11((big.as_pointer() as Ptr<i8>)) }) == 2_usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strlen(big) == 2"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strlen(void)"),
                    )
                });
            }
        }
    };
    (*big.borrow_mut())[(2) as usize] = (('x' as i32) as i8);
    (*big.borrow_mut())[(3) as usize] = (('\0' as i32) as i8);
    {
        &(4usize);
        {
            if (((({ strlen_11((big.as_pointer() as Ptr<i8>)) }) == 3_usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strlen(big) == 3"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        59_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strlen(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strcmp_13() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abc");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abd");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('c' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    {
        &(4usize);
        {
            if (((({
                let ___s1: Ptr<i8> = (p).clone();
                let ___s2: Ptr<i8> = (p).clone();
                strcmp_12(___s1, ___s2)
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(p, p) == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        69_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcmp(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strcmp_12((p).clone(), (q).clone()) }) < 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(p, q) < 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        70_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcmp(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strcmp_12((buf.as_pointer() as Ptr<i8>), (p).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, p) == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        71_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcmp(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strncmp_14() {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcdef");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcxyz");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('c' as i32) as i8),
        (('d' as i32) as i8),
        (('e' as i32) as i8),
        (('f' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    let mut n: usize = 3_usize;
    {
        &(4usize);
        {
            if (((({ strncmp_15((p).clone(), (q).clone(), n) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strncmp(p, q, n) == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        82_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strncmp(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strncmp_15((p).clone(), (q).clone(), (n).wrapping_add(1_usize)) }) < 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strncmp(p, q, n + 1) < 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        83_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strncmp(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strncmp_15((buf.as_pointer() as Ptr<i8>), (p).clone(), 6_usize) }) == 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strncmp(buf, p, 6) == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        84_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strncmp(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_memchr_16() {
    let data: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([16_i8, 32_i8, 48_i8, 64_i8])));
    let mut r: AnyPtr = ({
        memchr_17(
            ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            48,
            4_usize,
        )
    });
    {
        &(4usize);
        {
            if ((({ (r).clone() } == {
                (((data.as_pointer() as Ptr<i8>).offset(2)) as Ptr<i8>).to_any()
            }) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == &data[2]"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        90_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memchr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        { {} }
    };
    let mut p: AnyPtr = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut n: usize = 4_usize;
    {
        &(4usize);
        {
            if ((({ ({ memchr_17((p).clone(), 16, n) }) } == { (p).clone() }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"memchr(p, 0x10, n) == p"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        94_u32,
                        Ptr::<i8>::from_string_literal(b"void test_memchr(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strrchr_18() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strrchr_19(s.reinterpret_cast::<i8>(), ('l' as i32)) });
    {
        &(4usize);
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r != NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        100_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strrchr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((r.read()) as i32) == ('l' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"*r == \'l\'"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        101_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strrchr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (r).clone() } == { s.offset((9) as isize) }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == s + 9"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        102_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strrchr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strrchr_19(s.reinterpret_cast::<i8>(), ('z' as i32)) }).is_null()) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strrchr((char *)s, \'z\') == NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        103_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strrchr(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('a' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    {
        &(4usize);
        {
            if (((({ strrchr_19((buf.as_pointer() as Ptr<i8>), ('a' as i32)) })
                == ((buf.as_pointer() as Ptr<i8>).offset(2))) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strrchr(buf, \'a\') == &buf[2]"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        105_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strrchr(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strcspn_20() {
    {
        &(4usize);
        {
            if (((({
                strcspn_21(
                    Ptr::<i8>::from_string_literal(b"hello"),
                    Ptr::<i8>::from_string_literal(b"el"),
                )
            }) == 1_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcspn(\"hello\", \"el\") == 1"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        109_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcspn(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcspn_21(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"xyz"),
                )
            }) == 3_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcspn(\"abc\", \"xyz\") == 3"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        110_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcspn(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcspn_21(
                    Ptr::<i8>::from_string_literal(b""),
                    Ptr::<i8>::from_string_literal(b"abc"),
                )
            }) == 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcspn(\"\", \"abc\") == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        111_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcspn(void)"),
                    )
                });
            }
        }
    };
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut rej: Ptr<i8> = Ptr::<i8>::from_string_literal(b"el");
    {
        &(4usize);
        {
            if (((({ strcspn_21((s).clone(), (rej).clone()) }) == 1_usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcspn(s, rej) == 1"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        114_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcspn(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strspn_22() {
    {
        &(4usize);
        {
            if (((({
                strspn_23(
                    Ptr::<i8>::from_string_literal(b"hello"),
                    Ptr::<i8>::from_string_literal(b"hel"),
                )
            }) == 4_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strspn(\"hello\", \"hel\") == 4"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        118_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strspn(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strspn_23(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"xyz"),
                )
            }) == 0_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strspn(\"abc\", \"xyz\") == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        119_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strspn(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strspn_23(
                    Ptr::<i8>::from_string_literal(b"aaa"),
                    Ptr::<i8>::from_string_literal(b"a"),
                )
            }) == 3_usize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strspn(\"aaa\", \"a\") == 3"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        120_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strspn(void)"),
                    )
                });
            }
        }
    };
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut acc: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hel");
    {
        &(4usize);
        {
            if (((({ strspn_23((s).clone(), (acc).clone()) }) == 4_usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strspn(s, acc) == 4"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        123_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strspn(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strstr_24() {
    let mut h: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({
        strstr_25(
            h.reinterpret_cast::<i8>(),
            Ptr::<i8>::from_string_literal(b"world"),
        )
    });
    {
        &(4usize);
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r != NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        129_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strstr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (r).clone() } == { h.offset((6) as isize) }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == h + 6"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        130_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strstr(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strstr_25(
                    h.reinterpret_cast::<i8>(),
                    Ptr::<i8>::from_string_literal(b"xyz"),
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strstr((char *)h, \"xyz\") == NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        131_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strstr(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('h' as i32) as i8),
        (('e' as i32) as i8),
        (('l' as i32) as i8),
        (('l' as i32) as i8),
        (('o' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    {
        &(4usize);
        {
            if (((({
                strstr_25(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"ll"),
                )
            }) == ((buf.as_pointer() as Ptr<i8>).offset(2))) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strstr(buf, \"ll\") == &buf[2]"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        133_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strstr(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strpbrk_26() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({
        strpbrk_27(
            s.reinterpret_cast::<i8>(),
            Ptr::<i8>::from_string_literal(b"wo"),
        )
    });
    {
        &(4usize);
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r != NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        139_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strpbrk(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (r).clone() } == { s.offset((4) as isize) }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"r == s + 4"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        140_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strpbrk(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strpbrk_27(
                    s.reinterpret_cast::<i8>(),
                    Ptr::<i8>::from_string_literal(b"xyz"),
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strpbrk((char *)s, \"xyz\") == NULL"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        141_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strpbrk(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('c' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    {
        &(4usize);
        {
            if (((({
                strpbrk_27(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"b"),
                )
            }) == ((buf.as_pointer() as Ptr<i8>).offset(1))) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strpbrk(buf, \"b\") == &buf[1]"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        143_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strpbrk(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_strcasecmp_28() {
    {
        &(4usize);
        {
            if (((({
                strcasecmp_29(
                    Ptr::<i8>::from_string_literal(b"HELLO"),
                    Ptr::<i8>::from_string_literal(b"hello"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcasecmp(\"HELLO\", \"hello\") == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        147_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcasecmp(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcasecmp_29(
                    Ptr::<i8>::from_string_literal(b"abc"),
                    Ptr::<i8>::from_string_literal(b"abd"),
                )
            }) < 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcasecmp(\"abc\", \"abd\") < 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        148_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcasecmp(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcasecmp_29(
                    Ptr::<i8>::from_string_literal(b"abd"),
                    Ptr::<i8>::from_string_literal(b"abc"),
                )
            }) > 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcasecmp(\"abd\", \"abc\") > 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        149_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcasecmp(void)"),
                    )
                });
            }
        }
    };
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"FOO");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
    {
        &(4usize);
        {
            if (((({ strcasecmp_29((p).clone(), (q).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"strcasecmp(p, q) == 0"),
                        Ptr::<i8>::from_string_literal(b"string_h.c"),
                        152_u32,
                        Ptr::<i8>::from_string_literal(b"void test_strcasecmp(void)"),
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
    ({ test_memcpy_0() });
    ({ test_memset_3() });
    ({ test_memcmp_5() });
    ({ test_memmove_6() });
    ({ test_strchr_8() });
    ({ test_strlen_10() });
    ({ test_strcmp_13() });
    ({ test_strncmp_14() });
    ({ test_memchr_16() });
    ({ test_strrchr_18() });
    ({ test_strcspn_20() });
    ({ test_strspn_22() });
    ({ test_strstr_24() });
    ({ test_strpbrk_26() });
    ({ test_strcasecmp_28() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn memcpy_1(mut __dest: AnyPtr, mut __src: AnyPtr, mut __n: usize) -> AnyPtr {
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
pub fn memset_4(mut __s: AnyPtr, mut __c: i32, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn memmove_7(mut __dest: AnyPtr, mut __src: AnyPtr, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn strchr_9(mut __s: Ptr<i8>, mut __c: i32) -> Ptr<i8> {
    unimplemented!()
}
pub fn strlen_11(mut __s: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn strcmp_12(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn strncmp_15(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> i32 {
    unimplemented!()
}
pub fn memchr_17(mut __s: AnyPtr, mut __c: i32, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn strrchr_19(mut __s: Ptr<i8>, mut __c: i32) -> Ptr<i8> {
    unimplemented!()
}
pub fn strcspn_21(mut __s: Ptr<i8>, mut __reject: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn strspn_23(mut __s: Ptr<i8>, mut __accept: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn strstr_25(mut __haystack: Ptr<i8>, mut __needle: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strpbrk_27(mut __s: Ptr<i8>, mut __accept: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strcasecmp_29(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
