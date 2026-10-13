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
pub struct div_t {
    #[offset(0)]
    pub quot: i32,
    #[offset(4)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct ldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct lldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub fds_bits: Value<Box<[i64]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct random_data {
    #[offset(0)]
    #[byte_size(8)]
    pub fptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub rptr: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub state: Ptr<i32>,
    #[offset(24)]
    pub rand_type: i32,
    #[offset(28)]
    pub rand_deg: i32,
    #[offset(32)]
    pub rand_sep: i32,
    #[offset(40)]
    #[byte_size(8)]
    pub end_ptr: Ptr<i32>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct drand48_data {
    #[offset(0)]
    #[byte_size(6)]
    pub __x: Value<Box<[u16]>>,
    #[offset(6)]
    #[byte_size(6)]
    pub __old_x: Value<Box<[u16]>>,
    #[offset(12)]
    pub __c: u16,
    #[offset(14)]
    pub __init: u16,
    #[offset(16)]
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
pub fn test_memcpy_1() {
    let src: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hello\0")));
    let dst: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
    let mut r: AnyPtr = ({
        memcpy_2(
            ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ((src.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            6_usize,
        )
    });
    (if ({ (r).clone() } == { ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == dst"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memcpy()"),
            )
        });
    });
    (if ((((*dst.borrow())[(0) as usize] as i32) == (('h' as i8) as i32))
        && (((*dst.borrow())[(1) as usize] as i32) == (('e' as i8) as i32)))
        && (((*dst.borrow())[(2) as usize] as i32) == (('l' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(
                    b"dst[0] == \'h\' && dst[1] == \'e\' && dst[2] == \'l\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memcpy()"),
            )
        });
    });
    (if ((((*dst.borrow())[(3) as usize] as i32) == (('l' as i8) as i32))
        && (((*dst.borrow())[(4) as usize] as i32) == (('o' as i8) as i32)))
        && (((*dst.borrow())[(5) as usize] as i32) == (('\0' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(
                    b"dst[3] == \'l\' && dst[4] == \'o\' && dst[5] == \'\\0\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memcpy()"),
            )
        });
    });
}
pub fn test_memset_4() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut r: AnyPtr = ({
        memset_5(
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            (('x' as i8) as i32),
            4_usize,
        )
    });
    (if ({ (r).clone() } == { ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any() }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == buf"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memset()"),
            )
        });
    });
    (if (((((*buf.borrow())[(0) as usize] as i32) == (('x' as i8) as i32))
        && (((*buf.borrow())[(1) as usize] as i32) == (('x' as i8) as i32)))
        && (((*buf.borrow())[(2) as usize] as i32) == (('x' as i8) as i32)))
        && (((*buf.borrow())[(3) as usize] as i32) == (('x' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(
                    b"buf[0] == \'x\' && buf[1] == \'x\' && buf[2] == \'x\' && buf[3] == \'x\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memset()"),
            )
        });
    });
}
pub fn test_memcmp_6() {
    let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let b: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let c: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 9_i8, 4_i8])));
    (if (({
        memcmp_7(
            ((a.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ((b.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            4_usize,
        )
    }) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::memcmp(a, b, 4) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memcmp()"),
            )
        });
    });
    (if (({
        memcmp_7(
            ((a.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ((c.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            4_usize,
        )
    }) < 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::memcmp(a, c, 4) < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memcmp()"),
            )
        });
    });
    (if (({
        memcmp_7(
            ((c.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            ((a.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            4_usize,
        )
    }) > 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::memcmp(c, a, 4) > 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memcmp()"),
            )
        });
    });
}
pub fn test_memmove_8() {
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
        memmove_9(___dest, ___src, 4_usize)
    });
    (if ({ (r).clone() } == {
        ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any()
    }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == buf + 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memmove()"),
            )
        });
    });
    (if ((((*buf.borrow())[(0) as usize] as i32) == (('a' as i8) as i32))
        && (((*buf.borrow())[(1) as usize] as i32) == (('a' as i8) as i32)))
        && (((*buf.borrow())[(2) as usize] as i32) == (('b' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(
                    b"buf[0] == \'a\' && buf[1] == \'a\' && buf[2] == \'b\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memmove()"),
            )
        });
    });
    (if ((((*buf.borrow())[(3) as usize] as i32) == (('c' as i8) as i32))
        && (((*buf.borrow())[(4) as usize] as i32) == (('d' as i8) as i32)))
        && (((*buf.borrow())[(5) as usize] as i32) == (('\0' as i8) as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(
                    b"buf[3] == \'c\' && buf[4] == \'d\' && buf[5] == \'\\0\'",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memmove()"),
            )
        });
    });
}
pub fn test_strchr_10() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strchr_11((s).clone(), (('w' as i8) as i32)) });
    (if !((r).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strchr()"),
            )
        });
    });
    (if (((r.read()) as i32) == (('w' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"*r == \'w\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strchr()"),
            )
        });
    });
    (if ({ strchr_11((s).clone(), (('z' as i8) as i32)) }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strchr(s, \'z\') == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strchr()"),
            )
        });
    });
}
pub fn test_strlen_12() {
    (if (({ strlen_13(Ptr::<i8>::from_string_literal(b"")) }) == 0_usize) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strlen(\"\") == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strlen()"),
            )
        });
    });
    (if (({ strlen_13(Ptr::<i8>::from_string_literal(b"hello")) }) == 5_usize) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strlen(\"hello\") == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strlen()"),
            )
        });
    });
    (if (({ strlen_13(Ptr::<i8>::from_string_literal(b"hello world")) }) == 11_usize) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strlen(\"hello world\") == 11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strlen()"),
            )
        });
    });
}
pub fn test_strcmp_14() {
    (if (({
        strcmp_15(
            Ptr::<i8>::from_string_literal(b"abc"),
            Ptr::<i8>::from_string_literal(b"abc"),
        )
    }) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(\"abc\", \"abc\") == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcmp()"),
            )
        });
    });
    (if (({
        strcmp_15(
            Ptr::<i8>::from_string_literal(b"abc"),
            Ptr::<i8>::from_string_literal(b"abd"),
        )
    }) < 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(\"abc\", \"abd\") < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcmp()"),
            )
        });
    });
    (if (({
        strcmp_15(
            Ptr::<i8>::from_string_literal(b"abd"),
            Ptr::<i8>::from_string_literal(b"abc"),
        )
    }) > 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(\"abd\", \"abc\") > 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcmp()"),
            )
        });
    });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abc");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abd");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    (if (({
        let ___s1: Ptr<i8> = (p).clone();
        let ___s2: Ptr<i8> = (p).clone();
        strcmp_15(___s1, ___s2)
    }) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(p, p) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcmp()"),
            )
        });
    });
    (if (({ strcmp_15((p).clone(), (q).clone()) }) < 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(p, q) < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcmp()"),
            )
        });
    });
    (if (({ strcmp_15((buf.as_pointer() as Ptr<i8>), (p).clone()) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(buf, p) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcmp()"),
            )
        });
    });
}
pub fn test_strncmp_16() {
    (if (({
        strncmp_17(
            Ptr::<i8>::from_string_literal(b"abcdef"),
            Ptr::<i8>::from_string_literal(b"abcxyz"),
            3_usize,
        )
    }) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strncmp(\"abcdef\", \"abcxyz\", 3) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strncmp()"),
            )
        });
    });
    (if (({
        strncmp_17(
            Ptr::<i8>::from_string_literal(b"abcdef"),
            Ptr::<i8>::from_string_literal(b"abcxyz"),
            4_usize,
        )
    }) < 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strncmp(\"abcdef\", \"abcxyz\", 4) < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strncmp()"),
            )
        });
    });
    (if (({
        strncmp_17(
            Ptr::<i8>::from_string_literal(b"abcxyz"),
            Ptr::<i8>::from_string_literal(b"abcdef"),
            4_usize,
        )
    }) > 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strncmp(\"abcxyz\", \"abcdef\", 4) > 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strncmp()"),
            )
        });
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
    let mut n: usize = 3_usize;
    (if (({ strncmp_17((p).clone(), (q).clone(), n) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strncmp(p, q, n) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strncmp()"),
            )
        });
    });
    (if (({ strncmp_17((p).clone(), (q).clone(), (n).wrapping_add(1_usize)) }) < 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strncmp(p, q, n + 1) < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strncmp()"),
            )
        });
    });
    (if (({ strncmp_17((buf.as_pointer() as Ptr<i8>), (p).clone(), 6_usize) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strncmp(buf, p, 6) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strncmp()"),
            )
        });
    });
}
pub fn test_memchr_18() {
    let data: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([16_i8, 32_i8, 48_i8, 64_i8])));
    let mut r: AnyPtr = ({
        memchr_19(
            ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            48,
            4_usize,
        )
    });
    (if ({ (r).clone() } == { (((data.as_pointer() as Ptr<i8>).offset(2)) as Ptr<i8>).to_any() }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == &data[2]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memchr()"),
            )
        });
    });
    (if ({
        memchr_19(
            ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            153,
            4_usize,
        )
    })
    .is_null()
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::memchr(data, 0x99, 4) == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memchr()"),
            )
        });
    });
    let mut p: AnyPtr = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut n: usize = 4_usize;
    (if ({ ({ memchr_19((p).clone(), 16, n) }) } == { (p).clone() }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::memchr(p, 0x10, n) == p"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_memchr()"),
            )
        });
    });
}
pub fn test_strrchr_20() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strrchr_21((s).clone(), (('l' as i8) as i32)) });
    (if !((r).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strrchr()"),
            )
        });
    });
    (if (((r.read()) as i32) == (('l' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"*r == \'l\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strrchr()"),
            )
        });
    });
    (if ({ (r).clone() } == { s.offset((9) as isize) }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == s + 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strrchr()"),
            )
        });
    });
    (if ({ strrchr_21((s).clone(), (('z' as i8) as i32)) }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strrchr(s, \'z\') == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strrchr()"),
            )
        });
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('a' as i8),
        ('\0' as i8),
    ])));
    (if (({ strrchr_22((buf.as_pointer() as Ptr<i8>), (('a' as i8) as i32)) })
        == ((buf.as_pointer() as Ptr<i8>).offset(2)))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strrchr(buf, \'a\') == &buf[2]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strrchr()"),
            )
        });
    });
}
pub fn test_strdup_23() {
    let mut d: Ptr<i8> = ({ strdup_24(Ptr::<i8>::from_string_literal(b"hello")) });
    (if !((d).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"d != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strdup()"),
            )
        });
    });
    (if (({ strcmp_15((d).clone(), Ptr::<i8>::from_string_literal(b"hello")) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(d, \"hello\") == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strdup()"),
            )
        });
    });
    ({ free_25((d).to_any()) });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"world");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    let mut d2: Ptr<i8> = ({ strdup_24((p).clone()) });
    (if !((d2).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"d2 != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strdup()"),
            )
        });
    });
    (if (({ strcmp_15((d2).clone(), (p).clone()) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(d2, p) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strdup()"),
            )
        });
    });
    ({ free_25((d2).to_any()) });
    let mut d3: Ptr<i8> = ({ strdup_24((buf.as_pointer() as Ptr<i8>)) });
    (if !((d3).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"d3 != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strdup()"),
            )
        });
    });
    (if (({ strcmp_15((d3).clone(), (buf.as_pointer() as Ptr<i8>)) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcmp(d3, buf) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strdup()"),
            )
        });
    });
    ({ free_25((d3).to_any()) });
}
pub fn test_strcspn_26() {
    (if (({
        strcspn_27(
            Ptr::<i8>::from_string_literal(b"hello"),
            Ptr::<i8>::from_string_literal(b"el"),
        )
    }) == 1_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcspn(\"hello\", \"el\") == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcspn()"),
            )
        });
    });
    (if (({
        strcspn_27(
            Ptr::<i8>::from_string_literal(b"abc"),
            Ptr::<i8>::from_string_literal(b"xyz"),
        )
    }) == 3_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcspn(\"abc\", \"xyz\") == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcspn()"),
            )
        });
    });
    (if (({
        strcspn_27(
            Ptr::<i8>::from_string_literal(b""),
            Ptr::<i8>::from_string_literal(b"abc"),
        )
    }) == 0_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcspn(\"\", \"abc\") == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcspn()"),
            )
        });
    });
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut rej: Ptr<i8> = Ptr::<i8>::from_string_literal(b"el");
    (if (({ strcspn_27((s).clone(), (rej).clone()) }) == 1_usize) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strcspn(s, rej) == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcspn()"),
            )
        });
    });
}
pub fn test_strspn_28() {
    (if (({
        strspn_29(
            Ptr::<i8>::from_string_literal(b"hello"),
            Ptr::<i8>::from_string_literal(b"hel"),
        )
    }) == 4_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strspn(\"hello\", \"hel\") == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strspn()"),
            )
        });
    });
    (if (({
        strspn_29(
            Ptr::<i8>::from_string_literal(b"abc"),
            Ptr::<i8>::from_string_literal(b"xyz"),
        )
    }) == 0_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strspn(\"abc\", \"xyz\") == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strspn()"),
            )
        });
    });
    (if (({
        strspn_29(
            Ptr::<i8>::from_string_literal(b"aaa"),
            Ptr::<i8>::from_string_literal(b"a"),
        )
    }) == 3_usize)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strspn(\"aaa\", \"a\") == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strspn()"),
            )
        });
    });
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut acc: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hel");
    (if (({ strspn_29((s).clone(), (acc).clone()) }) == 4_usize) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strspn(s, acc) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strspn()"),
            )
        });
    });
}
pub fn test_strstr_30() {
    let mut h: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strstr_31((h).clone(), Ptr::<i8>::from_string_literal(b"world")) });
    (if !((r).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strstr()"),
            )
        });
    });
    (if ({ (r).clone() } == { h.offset((6) as isize) }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == h + 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strstr()"),
            )
        });
    });
    (if ({ strstr_31((h).clone(), Ptr::<i8>::from_string_literal(b"xyz")) }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strstr(h, \"xyz\") == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strstr()"),
            )
        });
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('h' as i8),
        ('e' as i8),
        ('l' as i8),
        ('l' as i8),
        ('o' as i8),
        ('\0' as i8),
    ])));
    (if (({
        strstr_32(
            (buf.as_pointer() as Ptr<i8>),
            Ptr::<i8>::from_string_literal(b"ll"),
        )
    }) == ((buf.as_pointer() as Ptr<i8>).offset(2)))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strstr(buf, \"ll\") == &buf[2]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strstr()"),
            )
        });
    });
}
pub fn test_strpbrk_33() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = ({ strpbrk_34((s).clone(), Ptr::<i8>::from_string_literal(b"wo")) });
    (if !((r).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strpbrk()"),
            )
        });
    });
    (if ({ (r).clone() } == { s.offset((4) as isize) }) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"r == s + 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strpbrk()"),
            )
        });
    });
    (if ({ strpbrk_34((s).clone(), Ptr::<i8>::from_string_literal(b"xyz")) }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strpbrk(s, \"xyz\") == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strpbrk()"),
            )
        });
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    (if (({
        strpbrk_35(
            (buf.as_pointer() as Ptr<i8>),
            Ptr::<i8>::from_string_literal(b"b"),
        )
    }) == ((buf.as_pointer() as Ptr<i8>).offset(1)))
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"std::strpbrk(buf, \"b\") == &buf[1]"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strpbrk()"),
            )
        });
    });
}
pub fn test_strcasecmp_36() {
    (if (({
        strcasecmp_37(
            Ptr::<i8>::from_string_literal(b"HELLO"),
            Ptr::<i8>::from_string_literal(b"hello"),
        )
    }) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"strcasecmp(\"HELLO\", \"hello\") == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcasecmp()"),
            )
        });
    });
    (if (({
        strcasecmp_37(
            Ptr::<i8>::from_string_literal(b"abc"),
            Ptr::<i8>::from_string_literal(b"abd"),
        )
    }) < 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"strcasecmp(\"abc\", \"abd\") < 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcasecmp()"),
            )
        });
    });
    (if (({
        strcasecmp_37(
            Ptr::<i8>::from_string_literal(b"abd"),
            Ptr::<i8>::from_string_literal(b"abc"),
        )
    }) > 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"strcasecmp(\"abd\", \"abc\") > 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcasecmp()"),
            )
        });
    });
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"FOO");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
    (if (({ strcasecmp_37((p).clone(), (q).clone()) }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"strcasecmp(p, q) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void test_strcasecmp()"),
            )
        });
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_memcpy_1() });
    ({ test_memset_4() });
    ({ test_memcmp_6() });
    ({ test_memmove_8() });
    ({ test_strchr_10() });
    ({ test_strlen_12() });
    ({ test_strcmp_14() });
    ({ test_strncmp_16() });
    ({ test_memchr_18() });
    ({ test_strrchr_20() });
    ({ test_strdup_23() });
    ({ test_strcspn_26() });
    ({ test_strspn_28() });
    ({ test_strstr_30() });
    ({ test_strpbrk_33() });
    ({ test_strcasecmp_36() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn memcpy_2(mut __dest: AnyPtr, mut __src: AnyPtr, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn memset_5(mut __s: AnyPtr, mut __c: i32, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn memcmp_7(mut __s1: AnyPtr, mut __s2: AnyPtr, mut __n: usize) -> i32 {
    unimplemented!()
}
pub fn memmove_9(mut __dest: AnyPtr, mut __src: AnyPtr, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn strchr_11(mut __s: Ptr<i8>, mut __c: i32) -> Ptr<i8> {
    unimplemented!()
}
pub fn strlen_13(mut __s: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn strcmp_15(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn strncmp_17(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>, mut __n: usize) -> i32 {
    unimplemented!()
}
pub fn memchr_19(mut __s: AnyPtr, mut __c: i32, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn strrchr_21(mut __s: Ptr<i8>, mut __c: i32) -> Ptr<i8> {
    unimplemented!()
}
pub fn strrchr_22(mut __s: Ptr<i8>, mut __c: i32) -> Ptr<i8> {
    unimplemented!()
}
pub fn strdup_24(mut __s: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn free_25(mut __ptr: AnyPtr) {
    unimplemented!()
}
pub fn strcspn_27(mut __s: Ptr<i8>, mut __reject: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn strspn_29(mut __s: Ptr<i8>, mut __accept: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn strstr_31(mut __haystack: Ptr<i8>, mut __needle: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strstr_32(mut __haystack: Ptr<i8>, mut __needle: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strpbrk_34(mut __s: Ptr<i8>, mut __accept: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strpbrk_35(mut __s: Ptr<i8>, mut __accept: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strcasecmp_37(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
