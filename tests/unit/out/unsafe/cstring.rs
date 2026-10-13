extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct div_t {
    pub quot: i32,
    pub rem: i32,
}
impl Clone for div_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for div_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ldiv_t {
    pub quot: i64,
    pub rem: i64,
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct lldiv_t {
    pub quot: i64,
    pub rem: i64,
}
impl Clone for lldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl lldiv_t {}
impl lldiv_t {}
impl lldiv_t {}
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
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub fds_bits: [i64; 16],
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_0,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_0 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: i32,
    pub rand_deg: i32,
    pub rand_sep: i32,
    pub end_ptr: *mut i32,
}
impl Clone for random_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for random_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct drand48_data {
    pub __x: [u16; 3],
    pub __old_x: [u16; 3],
    pub __c: u16,
    pub __init: u16,
    pub __a: u64,
}
impl Clone for drand48_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for drand48_data {
    fn default() -> Self {
        unimplemented!()
    }
}
pub unsafe fn test_memcpy_1() {
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
        memcpy_2(
            (dst.as_mut_ptr() as *mut ::libc::c_void),
            (src.as_ptr() as *const ::libc::c_void),
            6_usize,
        )
    });
    (if ((r) == (dst.as_mut_ptr() as *mut ::libc::c_void)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == dst".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memcpy()".as_ptr(),
            )
        });
    });
    (if (((dst[(0) as usize] as i32) == (('h' as libc::c_char) as i32))
        && ((dst[(1) as usize] as i32) == (('e' as libc::c_char) as i32)))
        && ((dst[(2) as usize] as i32) == (('l' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"dst[0] == \'h\' && dst[1] == \'e\' && dst[2] == \'l\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memcpy()".as_ptr(),
            )
        });
    });
    (if (((dst[(3) as usize] as i32) == (('l' as libc::c_char) as i32))
        && ((dst[(4) as usize] as i32) == (('o' as libc::c_char) as i32)))
        && ((dst[(5) as usize] as i32) == (('\0' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"dst[3] == \'l\' && dst[4] == \'o\' && dst[5] == \'\\0\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memcpy()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_memset_4() {
    let mut buf: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    let mut r: *mut ::libc::c_void = (unsafe {
        memset_5(
            (buf.as_mut_ptr() as *mut ::libc::c_void),
            (('x' as libc::c_char) as i32),
            4_usize,
        )
    });
    (if ((r) == (buf.as_mut_ptr() as *mut ::libc::c_void)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == buf".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memset()".as_ptr(),
            )
        });
    });
    (if ((((buf[(0) as usize] as i32) == (('x' as libc::c_char) as i32))
        && ((buf[(1) as usize] as i32) == (('x' as libc::c_char) as i32)))
        && ((buf[(2) as usize] as i32) == (('x' as libc::c_char) as i32)))
        && ((buf[(3) as usize] as i32) == (('x' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"buf[0] == \'x\' && buf[1] == \'x\' && buf[2] == \'x\' && buf[3] == \'x\'"
                    .as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memset()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_memcmp_6() {
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
    (if ((unsafe {
        memcmp_7(
            (a.as_ptr() as *const ::libc::c_void),
            (b.as_ptr() as *const ::libc::c_void),
            4_usize,
        )
    }) == (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::memcmp(a, b, 4) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memcmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        memcmp_7(
            (a.as_ptr() as *const ::libc::c_void),
            (c.as_ptr() as *const ::libc::c_void),
            4_usize,
        )
    }) < (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::memcmp(a, c, 4) < 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memcmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        memcmp_7(
            (c.as_ptr() as *const ::libc::c_void),
            (a.as_ptr() as *const ::libc::c_void),
            4_usize,
        )
    }) > (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::memcmp(c, a, 4) > 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memcmp()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_memmove_8() {
    let mut buf: [libc::c_char; 6] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('d' as libc::c_char),
        ('e' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    let mut r: *mut ::libc::c_void = (unsafe {
        let ___dest: *mut ::libc::c_void =
            (buf.as_mut_ptr().offset((1) as isize) as *mut ::libc::c_void);
        let ___src: *const ::libc::c_void = (buf.as_mut_ptr() as *const ::libc::c_void);
        memmove_9(___dest, ___src, 4_usize)
    });
    (if ((r) == (buf.as_mut_ptr().offset((1) as isize) as *mut ::libc::c_void)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == buf + 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memmove()".as_ptr(),
            )
        });
    });
    (if (((buf[(0) as usize] as i32) == (('a' as libc::c_char) as i32))
        && ((buf[(1) as usize] as i32) == (('a' as libc::c_char) as i32)))
        && ((buf[(2) as usize] as i32) == (('b' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"buf[0] == \'a\' && buf[1] == \'a\' && buf[2] == \'b\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memmove()".as_ptr(),
            )
        });
    });
    (if (((buf[(3) as usize] as i32) == (('c' as libc::c_char) as i32))
        && ((buf[(4) as usize] as i32) == (('d' as libc::c_char) as i32)))
        && ((buf[(5) as usize] as i32) == (('\0' as libc::c_char) as i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"buf[3] == \'c\' && buf[4] == \'d\' && buf[5] == \'\\0\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memmove()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strchr_10() {
    let mut s: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char = (unsafe { strchr_11(s, (('w' as libc::c_char) as i32)) });
    (if !((r).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strchr()".as_ptr(),
            )
        });
    });
    (if (((*r) as i32) == (('w' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"*r == \'w\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strchr()".as_ptr(),
            )
        });
    });
    (if (unsafe { strchr_11(s, (('z' as libc::c_char) as i32)) }).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strchr(s, \'z\') == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strchr()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strlen_12() {
    (if ((unsafe { strlen_13(c"".as_ptr()) }) == (0_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strlen(\"\") == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strlen()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strlen_13(c"hello".as_ptr()) }) == (5_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strlen(\"hello\") == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strlen()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strlen_13(c"hello world".as_ptr()) }) == (11_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strlen(\"hello world\") == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strlen()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strcmp_14() {
    (if ((unsafe { strcmp_15(c"abc".as_ptr(), c"abc".as_ptr()) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(\"abc\", \"abc\") == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15(c"abc".as_ptr(), c"abd".as_ptr()) }) < (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(\"abc\", \"abd\") < 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15(c"abd".as_ptr(), c"abc".as_ptr()) }) > (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(\"abd\", \"abc\") > 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcmp()".as_ptr(),
            )
        });
    });
    let mut p: *const libc::c_char = c"abc".as_ptr();
    let mut q: *const libc::c_char = c"abd".as_ptr();
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe {
        let ___s1: *const libc::c_char = p;
        let ___s2: *const libc::c_char = p;
        strcmp_15(___s1, ___s2)
    }) == (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(p, p) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15(p, q) }) < (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(p, q) < 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15((buf.as_mut_ptr()).cast_const(), p) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(buf, p) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcmp()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strncmp_16() {
    (if ((unsafe { strncmp_17(c"abcdef".as_ptr(), c"abcxyz".as_ptr(), 3_usize) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strncmp(\"abcdef\", \"abcxyz\", 3) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strncmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strncmp_17(c"abcdef".as_ptr(), c"abcxyz".as_ptr(), 4_usize) }) < (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strncmp(\"abcdef\", \"abcxyz\", 4) < 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strncmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strncmp_17(c"abcxyz".as_ptr(), c"abcdef".as_ptr(), 4_usize) }) > (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strncmp(\"abcxyz\", \"abcdef\", 4) > 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strncmp()".as_ptr(),
            )
        });
    });
    let mut p: *const libc::c_char = c"abcdef".as_ptr();
    let mut q: *const libc::c_char = c"abcxyz".as_ptr();
    let mut buf: [libc::c_char; 7] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('d' as libc::c_char),
        ('e' as libc::c_char),
        ('f' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    let mut n: usize = 3_usize;
    (if ((unsafe { strncmp_17(p, q, n) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strncmp(p, q, n) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strncmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strncmp_17(p, q, (n).wrapping_add(1_usize)) }) < (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strncmp(p, q, n + 1) < 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strncmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strncmp_17((buf.as_mut_ptr()).cast_const(), p, 6_usize) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strncmp(buf, p, 6) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strncmp()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_memchr_18() {
    let data: [libc::c_char; 4] = [
        (16 as libc::c_char),
        (32 as libc::c_char),
        (48 as libc::c_char),
        (64 as libc::c_char),
    ];
    let mut r: *const ::libc::c_void =
        (unsafe { memchr_19((data.as_ptr() as *const ::libc::c_void), 48, 4_usize) });
    (if ((r) == ((&data[(2) as usize] as *const libc::c_char) as *const ::libc::c_void)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == &data[2]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memchr()".as_ptr(),
            )
        });
    });
    (if (unsafe { memchr_19((data.as_ptr() as *const ::libc::c_void), 153, 4_usize) }).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::memchr(data, 0x99, 4) == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memchr()".as_ptr(),
            )
        });
    });
    let mut p: *const ::libc::c_void = (data.as_ptr() as *const ::libc::c_void);
    let mut n: usize = 4_usize;
    (if ((unsafe { memchr_19(p, 16, n) }) == (p)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::memchr(p, 0x10, n) == p".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_memchr()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strrchr_20() {
    let mut s: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char = (unsafe { strrchr_21(s, (('l' as libc::c_char) as i32)) });
    (if !((r).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strrchr()".as_ptr(),
            )
        });
    });
    (if (((*r) as i32) == (('l' as libc::c_char) as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"*r == \'l\'".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strrchr()".as_ptr(),
            )
        });
    });
    (if ((r) == (s.offset((9) as isize))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == s + 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strrchr()".as_ptr(),
            )
        });
    });
    (if (unsafe { strrchr_21(s, (('z' as libc::c_char) as i32)) }).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strrchr(s, \'z\') == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strrchr()".as_ptr(),
            )
        });
    });
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('a' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe { strrchr_22(buf.as_mut_ptr(), (('a' as libc::c_char) as i32)) })
        == (&mut buf[(2) as usize] as *mut libc::c_char))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strrchr(buf, \'a\') == &buf[2]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strrchr()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strdup_23() {
    let mut d: *mut libc::c_char = (unsafe { strdup_24(c"hello".as_ptr()) });
    (if !((d).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"d != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strdup()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15((d).cast_const(), c"hello".as_ptr()) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(d, \"hello\") == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strdup()".as_ptr(),
            )
        });
    });
    (unsafe { free_25((d as *mut ::libc::c_void)) });
    let mut p: *const libc::c_char = c"world".as_ptr();
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    let mut d2: *mut libc::c_char = (unsafe { strdup_24(p) });
    (if !((d2).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"d2 != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strdup()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15((d2).cast_const(), p) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(d2, p) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strdup()".as_ptr(),
            )
        });
    });
    (unsafe { free_25((d2 as *mut ::libc::c_void)) });
    let mut d3: *mut libc::c_char = (unsafe { strdup_24((buf.as_mut_ptr()).cast_const()) });
    (if !((d3).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"d3 != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strdup()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcmp_15((d3).cast_const(), (buf.as_mut_ptr()).cast_const()) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcmp(d3, buf) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strdup()".as_ptr(),
            )
        });
    });
    (unsafe { free_25((d3 as *mut ::libc::c_void)) });
}
pub unsafe fn test_strcspn_26() {
    (if ((unsafe { strcspn_27(c"hello".as_ptr(), c"el".as_ptr()) }) == (1_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcspn(\"hello\", \"el\") == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcspn()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcspn_27(c"abc".as_ptr(), c"xyz".as_ptr()) }) == (3_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcspn(\"abc\", \"xyz\") == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcspn()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcspn_27(c"".as_ptr(), c"abc".as_ptr()) }) == (0_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcspn(\"\", \"abc\") == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcspn()".as_ptr(),
            )
        });
    });
    let mut s: *const libc::c_char = c"hello".as_ptr();
    let mut rej: *const libc::c_char = c"el".as_ptr();
    (if ((unsafe { strcspn_27(s, rej) }) == (1_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strcspn(s, rej) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcspn()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strspn_28() {
    (if ((unsafe { strspn_29(c"hello".as_ptr(), c"hel".as_ptr()) }) == (4_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strspn(\"hello\", \"hel\") == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strspn()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strspn_29(c"abc".as_ptr(), c"xyz".as_ptr()) }) == (0_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strspn(\"abc\", \"xyz\") == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strspn()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strspn_29(c"aaa".as_ptr(), c"a".as_ptr()) }) == (3_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strspn(\"aaa\", \"a\") == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strspn()".as_ptr(),
            )
        });
    });
    let mut s: *const libc::c_char = c"hello".as_ptr();
    let mut acc: *const libc::c_char = c"hel".as_ptr();
    (if ((unsafe { strspn_29(s, acc) }) == (4_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strspn(s, acc) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strspn()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strstr_30() {
    let mut h: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char = (unsafe { strstr_31(h, c"world".as_ptr()) });
    (if !((r).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strstr()".as_ptr(),
            )
        });
    });
    (if ((r) == (h.offset((6) as isize))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == h + 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strstr()".as_ptr(),
            )
        });
    });
    (if (unsafe { strstr_31(h, c"xyz".as_ptr()) }).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strstr(h, \"xyz\") == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strstr()".as_ptr(),
            )
        });
    });
    let mut buf: [libc::c_char; 6] = [
        ('h' as libc::c_char),
        ('e' as libc::c_char),
        ('l' as libc::c_char),
        ('l' as libc::c_char),
        ('o' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe { strstr_32(buf.as_mut_ptr(), c"ll".as_ptr()) })
        == (&mut buf[(2) as usize] as *mut libc::c_char))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strstr(buf, \"ll\") == &buf[2]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strstr()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strpbrk_33() {
    let mut s: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char = (unsafe { strpbrk_34(s, c"wo".as_ptr()) });
    (if !((r).is_null()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strpbrk()".as_ptr(),
            )
        });
    });
    (if ((r) == (s.offset((4) as isize))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"r == s + 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strpbrk()".as_ptr(),
            )
        });
    });
    (if (unsafe { strpbrk_34(s, c"xyz".as_ptr()) }).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strpbrk(s, \"xyz\") == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strpbrk()".as_ptr(),
            )
        });
    });
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    (if ((unsafe { strpbrk_35(buf.as_mut_ptr(), c"b".as_ptr()) })
        == (&mut buf[(1) as usize] as *mut libc::c_char))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"std::strpbrk(buf, \"b\") == &buf[1]".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strpbrk()".as_ptr(),
            )
        });
    });
}
pub unsafe fn test_strcasecmp_36() {
    (if ((unsafe { strcasecmp_37(c"HELLO".as_ptr(), c"hello".as_ptr()) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"strcasecmp(\"HELLO\", \"hello\") == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcasecmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcasecmp_37(c"abc".as_ptr(), c"abd".as_ptr()) }) < (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"strcasecmp(\"abc\", \"abd\") < 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcasecmp()".as_ptr(),
            )
        });
    });
    (if ((unsafe { strcasecmp_37(c"abd".as_ptr(), c"abc".as_ptr()) }) > (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"strcasecmp(\"abd\", \"abc\") > 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcasecmp()".as_ptr(),
            )
        });
    });
    let mut p: *const libc::c_char = c"FOO".as_ptr();
    let mut q: *const libc::c_char = c"foo".as_ptr();
    (if ((unsafe { strcasecmp_37(p, q) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"strcasecmp(p, q) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_strcasecmp()".as_ptr(),
            )
        });
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_memcpy_1() });
    (unsafe { test_memset_4() });
    (unsafe { test_memcmp_6() });
    (unsafe { test_memmove_8() });
    (unsafe { test_strchr_10() });
    (unsafe { test_strlen_12() });
    (unsafe { test_strcmp_14() });
    (unsafe { test_strncmp_16() });
    (unsafe { test_memchr_18() });
    (unsafe { test_strrchr_20() });
    (unsafe { test_strdup_23() });
    (unsafe { test_strcspn_26() });
    (unsafe { test_strspn_28() });
    (unsafe { test_strstr_30() });
    (unsafe { test_strpbrk_33() });
    (unsafe { test_strcasecmp_36() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn memcpy_2(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn memset_5(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn memcmp_7(
    mut __s1: *const ::libc::c_void,
    mut __s2: *const ::libc::c_void,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn memmove_9(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn strchr_11(mut __s: *const libc::c_char, mut __c: i32) -> *const libc::c_char {
    unimplemented!()
}
pub unsafe fn strlen_13(mut __s: *const libc::c_char) -> usize {
    unimplemented!()
}
pub unsafe fn strcmp_15(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn strncmp_17(
    mut __s1: *const libc::c_char,
    mut __s2: *const libc::c_char,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn memchr_19(
    mut __s: *const ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *const ::libc::c_void {
    unimplemented!()
}
pub unsafe fn strrchr_21(mut __s: *const libc::c_char, mut __c: i32) -> *const libc::c_char {
    unimplemented!()
}
pub unsafe fn strrchr_22(mut __s: *mut libc::c_char, mut __c: i32) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strdup_24(mut __s: *const libc::c_char) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn free_25(mut __ptr: *mut ::libc::c_void) {
    unimplemented!()
}
pub unsafe fn strcspn_27(mut __s: *const libc::c_char, mut __reject: *const libc::c_char) -> usize {
    unimplemented!()
}
pub unsafe fn strspn_29(mut __s: *const libc::c_char, mut __accept: *const libc::c_char) -> usize {
    unimplemented!()
}
pub unsafe fn strstr_31(
    mut __haystack: *const libc::c_char,
    mut __needle: *const libc::c_char,
) -> *const libc::c_char {
    unimplemented!()
}
pub unsafe fn strstr_32(
    mut __haystack: *mut libc::c_char,
    mut __needle: *const libc::c_char,
) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strpbrk_34(
    mut __s: *const libc::c_char,
    mut __accept: *const libc::c_char,
) -> *const libc::c_char {
    unimplemented!()
}
pub unsafe fn strpbrk_35(
    mut __s: *mut libc::c_char,
    mut __accept: *const libc::c_char,
) -> *mut libc::c_char {
    unimplemented!()
}
pub unsafe fn strcasecmp_37(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
