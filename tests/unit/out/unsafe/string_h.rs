extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
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
pub unsafe fn test_memcpy_0() {
    let src: [libc::c_char; 6] = std::mem::transmute(*b"hello\0");
    let mut dst: [libc::c_char; 6] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    let mut r: *mut ::libc::c_void = (unsafe {
        memcpy_1(
            (dst.as_mut_ptr() as *mut ::libc::c_void),
            (src.as_ptr() as *const ::libc::c_void),
            6_usize,
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (dst.as_mut_ptr() as *mut ::libc::c_void)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == dst".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"void test_memcpy(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((dst[(0) as usize] as i32) == ('h' as i32)) as i32) != 0)
                && ((((dst[(1) as usize] as i32) == ('e' as i32)) as i32) != 0))
                as i32)
                != 0)
                && ((((dst[(2) as usize] as i32) == ('l' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"dst[0] == \'h\' && dst[1] == \'e\' && dst[2] == \'l\'"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"void test_memcpy(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((dst[(3) as usize] as i32) == ('l' as i32)) as i32) != 0)
                && ((((dst[(4) as usize] as i32) == ('o' as i32)) as i32) != 0))
                as i32)
                != 0)
                && ((((dst[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"dst[3] == \'l\' && dst[4] == \'o\' && dst[5] == \'\\0\'"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        12_u32,
                        c"void test_memcpy(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_memset_3() {
    let mut buf: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    let mut r: *mut ::libc::c_void = (unsafe {
        memset_4(
            (buf.as_mut_ptr() as *mut ::libc::c_void),
            ('x' as i32),
            4_usize,
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (buf.as_mut_ptr() as *mut ::libc::c_void)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == buf".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        18_u32,
                        c"void test_memset(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((((((((((buf[(0) as usize] as i32) == ('x' as i32)) as i32) != 0)
                && ((((buf[(1) as usize] as i32) == ('x' as i32)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(2) as usize] as i32) == ('x' as i32)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(3) as usize] as i32) == ('x' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2 ( ( c"buf[0] == \'x\' && buf[1] == \'x\' && buf[2] == \'x\' && buf[3] == \'x\'" .as_ptr() .cast_mut() ) .cast_const() , ( c"string_h.c" .as_ptr() .cast_mut() ) .cast_const() , 19_u32 , c"void test_memset(void)" .as_ptr() , )
                });
            }
        }
    };
}
pub unsafe fn test_memcmp_5() {
    let a: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    let b: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    let c: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (9 as libc::c_char),
        (4 as libc::c_char),
    ];
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
}
pub unsafe fn test_memmove_6() {
    let mut buf: [libc::c_char; 6] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
        (('c' as i32) as libc::c_char),
        (('d' as i32) as libc::c_char),
        (('e' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    let mut r: *mut ::libc::c_void = (unsafe {
        let ___dest: *mut ::libc::c_void =
            (buf.as_mut_ptr().offset((1) as isize) as *mut ::libc::c_void);
        let ___src: *const ::libc::c_void = (buf.as_mut_ptr() as *const ::libc::c_void);
        memmove_7(___dest, ___src, 4_usize)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (buf.as_mut_ptr().offset((1) as isize) as *mut ::libc::c_void)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == buf + 1".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        34_u32,
                        c"void test_memmove(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((buf[(0) as usize] as i32) == ('a' as i32)) as i32) != 0)
                && ((((buf[(1) as usize] as i32) == ('a' as i32)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(2) as usize] as i32) == ('b' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"buf[0] == \'a\' && buf[1] == \'a\' && buf[2] == \'b\'"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        35_u32,
                        c"void test_memmove(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((buf[(3) as usize] as i32) == ('c' as i32)) as i32) != 0)
                && ((((buf[(4) as usize] as i32) == ('d' as i32)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"buf[3] == \'c\' && buf[4] == \'d\' && buf[5] == \'\\0\'"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        36_u32,
                        c"void test_memmove(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strchr_8() {
    let mut s: *const libc::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut libc::c_char =
        (unsafe { strchr_9((s as *mut libc::c_char).cast_const(), ('w' as i32)) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
                        c"void test_strchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*r) as i32) == ('w' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"*r == \'w\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        43_u32,
                        c"void test_strchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strchr_9((s as *mut libc::c_char).cast_const(), ('z' as i32)) })
                .is_null()) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strchr((char *)s, \'z\') == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        44_u32,
                        c"void test_strchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strlen_10() {
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
    let buf: [libc::c_char; 8] = std::mem::transmute(*b"one\0two\0");
    let mut first: *const libc::c_char = buf.as_ptr();
    let mut second: *const libc::c_char =
        (&buf[((unsafe { strlen_11(first) }).wrapping_add(1_usize))] as *const libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strcmp_12(second, (c"two".as_ptr().cast_mut()).cast_const()) }) == (0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(second, \"two\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        54_u32,
                        c"void test_strlen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut big : [ libc::c_char ; 64] = std::mem::transmute(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") ;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strlen_11((big.as_mut_ptr()).cast_const()) }) == (2_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strlen(big) == 2".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        56_u32,
                        c"void test_strlen(void)".as_ptr(),
                    )
                });
            }
        }
    };
    big[(2) as usize] = (('x' as i32) as libc::c_char);
    big[(3) as usize] = (('\0' as i32) as libc::c_char);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strlen_11((big.as_mut_ptr()).cast_const()) }) == (3_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strlen(big) == 3".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        59_u32,
                        c"void test_strlen(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strcmp_13() {
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
    let mut p: *const libc::c_char = (c"abc".as_ptr().cast_mut()).cast_const();
    let mut q: *const libc::c_char = (c"abd".as_ptr().cast_mut()).cast_const();
    let mut buf: [libc::c_char; 4] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
        (('c' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                let ___s1: *const libc::c_char = p;
                let ___s2: *const libc::c_char = p;
                strcmp_12(___s1, ___s2)
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(p, p) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        69_u32,
                        c"void test_strcmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strcmp_12(p, q) }) < (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(p, q) < 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        70_u32,
                        c"void test_strcmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strcmp_12((buf.as_mut_ptr()).cast_const(), p) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(buf, p) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        71_u32,
                        c"void test_strcmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strncmp_14() {
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
    let mut p: *const libc::c_char = (c"abcdef".as_ptr().cast_mut()).cast_const();
    let mut q: *const libc::c_char = (c"abcxyz".as_ptr().cast_mut()).cast_const();
    let mut buf: [libc::c_char; 7] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
        (('c' as i32) as libc::c_char),
        (('d' as i32) as libc::c_char),
        (('e' as i32) as libc::c_char),
        (('f' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    let mut n: usize = 3_usize;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strncmp_15(p, q, n) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strncmp(p, q, n) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        82_u32,
                        c"void test_strncmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strncmp_15(p, q, (n).wrapping_add(1_usize)) }) < (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strncmp(p, q, n + 1) < 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        83_u32,
                        c"void test_strncmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strncmp_15((buf.as_mut_ptr()).cast_const(), p, 6_usize) }) == (0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strncmp(buf, p, 6) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        84_u32,
                        c"void test_strncmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_memchr_16() {
    let data: [libc::c_char; 4] = [
        (16 as libc::c_char),
        (32 as libc::c_char),
        (48 as libc::c_char),
        (64 as libc::c_char),
    ];
    let mut r: *mut ::libc::c_void =
        (unsafe { memchr_17((data.as_ptr() as *const ::libc::c_void), 48, 4_usize) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == ((&data[(2) as usize] as *const libc::c_char) as *mut ::libc::c_void))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == &data[2]".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        90_u32,
                        c"void test_memchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    let mut p: *const ::libc::c_void = (data.as_ptr() as *const ::libc::c_void);
    let mut n: usize = 4_usize;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { memchr_17(p, 16, n) }) == (p as *mut ::libc::c_void)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"memchr(p, 0x10, n) == p".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        94_u32,
                        c"void test_memchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strrchr_18() {
    let mut s: *const libc::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut libc::c_char =
        (unsafe { strrchr_19((s as *mut libc::c_char).cast_const(), ('l' as i32)) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        100_u32,
                        c"void test_strrchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*r) as i32) == ('l' as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"*r == \'l\'".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        101_u32,
                        c"void test_strrchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (s.offset((9) as isize) as *mut libc::c_char)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == s + 9".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        102_u32,
                        c"void test_strrchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strrchr_19((s as *mut libc::c_char).cast_const(), ('z' as i32)) })
                .is_null()) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strrchr((char *)s, \'z\') == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        103_u32,
                        c"void test_strrchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 4] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
        (('a' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strrchr_19((buf.as_mut_ptr()).cast_const(), ('a' as i32)) })
                == (&mut buf[(2) as usize] as *mut libc::c_char)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strrchr(buf, \'a\') == &buf[2]".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        105_u32,
                        c"void test_strrchr(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strcspn_20() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcspn_21(
                    (c"hello".as_ptr().cast_mut()).cast_const(),
                    (c"el".as_ptr().cast_mut()).cast_const(),
                )
            }) == (1_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcspn(\"hello\", \"el\") == 1".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        109_u32,
                        c"void test_strcspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcspn_21(
                    (c"abc".as_ptr().cast_mut()).cast_const(),
                    (c"xyz".as_ptr().cast_mut()).cast_const(),
                )
            }) == (3_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcspn(\"abc\", \"xyz\") == 3".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        110_u32,
                        c"void test_strcspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcspn_21(
                    (c"".as_ptr().cast_mut()).cast_const(),
                    (c"abc".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcspn(\"\", \"abc\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        111_u32,
                        c"void test_strcspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut s: *const libc::c_char = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut rej: *const libc::c_char = (c"el".as_ptr().cast_mut()).cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strcspn_21(s, rej) }) == (1_usize)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcspn(s, rej) == 1".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        114_u32,
                        c"void test_strcspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strspn_22() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strspn_23(
                    (c"hello".as_ptr().cast_mut()).cast_const(),
                    (c"hel".as_ptr().cast_mut()).cast_const(),
                )
            }) == (4_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strspn(\"hello\", \"hel\") == 4".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        118_u32,
                        c"void test_strspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strspn_23(
                    (c"abc".as_ptr().cast_mut()).cast_const(),
                    (c"xyz".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strspn(\"abc\", \"xyz\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        119_u32,
                        c"void test_strspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strspn_23(
                    (c"aaa".as_ptr().cast_mut()).cast_const(),
                    (c"a".as_ptr().cast_mut()).cast_const(),
                )
            }) == (3_usize)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strspn(\"aaa\", \"a\") == 3".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        120_u32,
                        c"void test_strspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut s: *const libc::c_char = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut acc: *const libc::c_char = (c"hel".as_ptr().cast_mut()).cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strspn_23(s, acc) }) == (4_usize)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strspn(s, acc) == 4".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        123_u32,
                        c"void test_strspn(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strstr_24() {
    let mut h: *const libc::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut libc::c_char = (unsafe {
        strstr_25(
            (h as *mut libc::c_char).cast_const(),
            (c"world".as_ptr().cast_mut()).cast_const(),
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        129_u32,
                        c"void test_strstr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (h.offset((6) as isize) as *mut libc::c_char)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == h + 6".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        130_u32,
                        c"void test_strstr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strstr_25(
                    (h as *mut libc::c_char).cast_const(),
                    (c"xyz".as_ptr().cast_mut()).cast_const(),
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strstr((char *)h, \"xyz\") == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        131_u32,
                        c"void test_strstr(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 6] = [
        (('h' as i32) as libc::c_char),
        (('e' as i32) as libc::c_char),
        (('l' as i32) as libc::c_char),
        (('l' as i32) as libc::c_char),
        (('o' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strstr_25(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"ll".as_ptr().cast_mut()).cast_const(),
                )
            }) == (&mut buf[(2) as usize] as *mut libc::c_char)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strstr(buf, \"ll\") == &buf[2]".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        133_u32,
                        c"void test_strstr(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strpbrk_26() {
    let mut s: *const libc::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut libc::c_char = (unsafe {
        strpbrk_27(
            (s as *mut libc::c_char).cast_const(),
            (c"wo".as_ptr().cast_mut()).cast_const(),
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((r).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        139_u32,
                        c"void test_strpbrk(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((r) == (s.offset((4) as isize) as *mut libc::c_char)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"r == s + 4".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        140_u32,
                        c"void test_strpbrk(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strpbrk_27(
                    (s as *mut libc::c_char).cast_const(),
                    (c"xyz".as_ptr().cast_mut()).cast_const(),
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strpbrk((char *)s, \"xyz\") == NULL".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        141_u32,
                        c"void test_strpbrk(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut buf: [libc::c_char; 4] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
        (('c' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strpbrk_27(
                    (buf.as_mut_ptr()).cast_const(),
                    (c"b".as_ptr().cast_mut()).cast_const(),
                )
            }) == (&mut buf[(1) as usize] as *mut libc::c_char)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strpbrk(buf, \"b\") == &buf[1]".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        143_u32,
                        c"void test_strpbrk(void)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn test_strcasecmp_28() {
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcasecmp_29(
                    (c"HELLO".as_ptr().cast_mut()).cast_const(),
                    (c"hello".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcasecmp(\"HELLO\", \"hello\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        147_u32,
                        c"void test_strcasecmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcasecmp_29(
                    (c"abc".as_ptr().cast_mut()).cast_const(),
                    (c"abd".as_ptr().cast_mut()).cast_const(),
                )
            }) < (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcasecmp(\"abc\", \"abd\") < 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        148_u32,
                        c"void test_strcasecmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcasecmp_29(
                    (c"abd".as_ptr().cast_mut()).cast_const(),
                    (c"abc".as_ptr().cast_mut()).cast_const(),
                )
            }) > (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcasecmp(\"abd\", \"abc\") > 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        149_u32,
                        c"void test_strcasecmp(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut p: *const libc::c_char = (c"FOO".as_ptr().cast_mut()).cast_const();
    let mut q: *const libc::c_char = (c"foo".as_ptr().cast_mut()).cast_const();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { strcasecmp_29(p, q) }) == (0)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcasecmp(p, q) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"string_h.c".as_ptr().cast_mut()).cast_const(),
                        152_u32,
                        c"void test_strcasecmp(void)".as_ptr(),
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
    (unsafe { test_memcpy_0() });
    (unsafe { test_memset_3() });
    (unsafe { test_memcmp_5() });
    (unsafe { test_memmove_6() });
    (unsafe { test_strchr_8() });
    (unsafe { test_strlen_10() });
    (unsafe { test_strcmp_13() });
    (unsafe { test_strncmp_14() });
    (unsafe { test_memchr_16() });
    (unsafe { test_strrchr_18() });
    (unsafe { test_strcspn_20() });
    (unsafe { test_strspn_22() });
    (unsafe { test_strstr_24() });
    (unsafe { test_strpbrk_26() });
    (unsafe { test_strcasecmp_28() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn memcpy_1(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
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
pub unsafe fn memset_4(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn memmove_7(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn strchr_9(mut __s: *const libc::c_char, mut __c: i32) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strlen_11(mut __s: *const libc::c_char) -> usize {
    unimplemented!()
}
pub unsafe fn strcmp_12(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn strncmp_15(
    mut __s1: *const libc::c_char,
    mut __s2: *const libc::c_char,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn memchr_17(
    mut __s: *const ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn strrchr_19(mut __s: *const libc::c_char, mut __c: i32) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strcspn_21(mut __s: *const libc::c_char, mut __reject: *const libc::c_char) -> usize {
    unimplemented!()
}
pub unsafe fn strspn_23(mut __s: *const libc::c_char, mut __accept: *const libc::c_char) -> usize {
    unimplemented!()
}
pub unsafe fn strstr_25(
    mut __haystack: *const libc::c_char,
    mut __needle: *const libc::c_char,
) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strpbrk_27(
    mut __s: *const libc::c_char,
    mut __accept: *const libc::c_char,
) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strcasecmp_29(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
