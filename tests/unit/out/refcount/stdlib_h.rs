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
    pub __fds_bits: Value<Box<[i64]>>,
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
pub fn test_setenv_getenv_1() {
    {
        &(4usize);
        {
            if (((({
                setenv_2(
                    Ptr::<i8>::from_string_literal(b"CPP2RUST_TEST_VAR"),
                    Ptr::<i8>::from_string_literal(b"test_value"),
                    1,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"setenv(\"CPP2RUST_TEST_VAR\", \"test_value\", 1) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"void test_setenv_getenv(void)"),
                    )
                });
            }
        }
    };
    let mut v: Ptr<i8> = ({ getenv_4(Ptr::<i8>::from_string_literal(b"CPP2RUST_TEST_VAR")) });
    {
        &(4usize);
        {
            if (((!((v).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"v != NULL"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"void test_setenv_getenv(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strcmp_5((v).clone(), Ptr::<i8>::from_string_literal(b"test_value")) }) == 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"strcmp(v, \"test_value\") == 0"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"void test_setenv_getenv(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                setenv_2(
                    Ptr::<i8>::from_string_literal(b"CPP2RUST_TEST_VAR"),
                    Ptr::<i8>::from_string_literal(b"replaced"),
                    1,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"setenv(\"CPP2RUST_TEST_VAR\", \"replaced\", 1) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"void test_setenv_getenv(void)"),
                    )
                });
            }
        }
    };
    v = ({ getenv_4(Ptr::<i8>::from_string_literal(b"CPP2RUST_TEST_VAR")) });
    {
        &(4usize);
        {
            if (((!((v).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"v != NULL"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"void test_setenv_getenv(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strcmp_5((v).clone(), Ptr::<i8>::from_string_literal(b"replaced")) }) == 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"strcmp(v, \"replaced\") == 0"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"void test_setenv_getenv(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_realpath_6() {
    let buf: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..4096).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        &(4usize);
        {
            if (((!(({
                realpath_7(
                    Ptr::<i8>::from_string_literal(b"/"),
                    (buf.as_pointer() as Ptr<i8>),
                )
            })
            .is_null())) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"realpath(\"/\", buf) != NULL"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        19_u32,
                        Ptr::<i8>::from_string_literal(b"void test_realpath(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                strcmp_5(
                    (buf.as_pointer() as Ptr<i8>),
                    Ptr::<i8>::from_string_literal(b"/"),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"strcmp(buf, \"/\") == 0"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        20_u32,
                        Ptr::<i8>::from_string_literal(b"void test_realpath(void)"),
                    )
                });
            }
        }
    };
    let mut p: Ptr<i8> = ({ realpath_7(Ptr::<i8>::from_string_literal(b"/"), Ptr::<i8>::null()) });
    {
        &(4usize);
        {
            if (((!((p).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"p != NULL"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        22_u32,
                        Ptr::<i8>::from_string_literal(b"void test_realpath(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strcmp_5((p).clone(), Ptr::<i8>::from_string_literal(b"/")) }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"strcmp(p, \"/\") == 0"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        23_u32,
                        Ptr::<i8>::from_string_literal(b"void test_realpath(void)"),
                    )
                });
            }
        }
    };
    ({ free_8((p).to_any()) });
    ({ __errno_location_9() }).write(0);
    {
        &(4usize);
        {
            if (((({
                realpath_7(
                    Ptr::<i8>::from_string_literal(b"/cpp2rust_definitely_missing"),
                    (buf.as_pointer() as Ptr<i8>),
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(
                            b"realpath(\"/cpp2rust_definitely_missing\", buf) == NULL",
                        ),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        26_u32,
                        Ptr::<i8>::from_string_literal(b"void test_realpath(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((({ __errno_location_9() }).read()) == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_3(
                        Ptr::<i8>::from_string_literal(b"errno == ENOENT"),
                        Ptr::<i8>::from_string_literal(b"stdlib_h.c"),
                        27_u32,
                        Ptr::<i8>::from_string_literal(b"void test_realpath(void)"),
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
    ({ test_setenv_getenv_1() });
    ({ test_realpath_6() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn setenv_2(mut __name: Ptr<i8>, mut __value: Ptr<i8>, mut __replace: i32) -> i32 {
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
pub fn getenv_4(mut __name: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn strcmp_5(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn realpath_7(mut __name: Ptr<i8>, mut __resolved: Ptr<i8>) -> Ptr<i8> {
    unimplemented!()
}
pub fn free_8(mut __ptr: AnyPtr) {
    unimplemented!()
}
pub fn __errno_location_9() -> Ptr<i32> {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
