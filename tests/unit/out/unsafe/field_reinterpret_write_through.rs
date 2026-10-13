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
    pub __fds_bits: [i64; 16],
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct In {
    pub a: i16,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub x: i32,
    pub in_: In,
    pub bytes: [u8; 4],
    pub arr: [i32; 3],
    pub tail: i64,
}
pub unsafe fn set_bytes_1(mut p: *mut u8, mut n: usize, mut v: u8) {
    let mut i: usize = 0_usize;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        (*p.offset((i) as isize)) = v;
        i.postfix_inc();
    }
}
pub unsafe fn all_bytes_2(mut p: *const u8, mut n: usize, mut v: u8) -> i32 {
    let mut i: usize = 0_usize;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        if (((((*p.offset((i) as isize)) as i32) != (v as i32)) as i32) != 0) {
            return 0;
        }
        i.postfix_inc();
    }
    return 1;
}
pub unsafe fn check_final_3(mut s: *mut S) {
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).x)
                == (((72340172838076673_u64 as u64).wrapping_mul((((19) as u8) as u64))) as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->x == REP(int, 0x13)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        62_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).in_.a as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((35) as u8) as u64))) as i16)
                    as i32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->in.a == REP(short, 0x23)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        63_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul((((51) as u8) as u64))) as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->in.b == REP(int, 0x33)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        64_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*s).bytes[(0) as usize] as i32) == (67)) as i32) != 0)
                && (((((*s).bytes[(1) as usize] as i32) == (83)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->bytes[0] == 0x43 && s->bytes[1] == 0x53"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        65_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*s).bytes[(2) as usize] as i32) == (99)) as i32) != 0)
                && (((((*s).bytes[(3) as usize] as i32) == (115)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->bytes[2] == 0x63 && s->bytes[3] == 0x73"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        66_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*s).arr[(0) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((131) as u8) as u64))) as i32))
                as i32)
                != 0)
                && (((((*s).arr[(1) as usize])
                    == (((72340172838076673_u64 as u64).wrapping_mul((((147) as u8) as u64)))
                        as i32)) as i32)
                    != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->arr[0] == REP(int, 0x83) && s->arr[1] == REP(int, 0x93)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        67_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).arr[(2) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((11) as u8) as u64))) as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->arr[2] == REP(int, 0x0b)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        68_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).tail)
                == (((72340172838076673_u64 as u64).wrapping_mul((((27) as u8) as u64))) as i64))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->tail == REP(long, 0x1b)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        69_u32,
                        c"void check_final(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
}
pub unsafe fn check_struct_5(mut s: *mut S) {
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).x as *mut i32) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset((::std::mem::offset_of!(S, x)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), ((16) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).x)
                    == (((72340172838076673_u64 as u64).wrapping_mul((((16) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->x == REP(int, (0x10))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), ((16) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x10))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).x =
            (((72340172838076673_u64 as u64).wrapping_mul(((((16) + (1)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((16) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x10) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((16) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x10) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((16) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).x)
                    == (((72340172838076673_u64 as u64).wrapping_mul(((((16) + (2)) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->x == REP(int, (0x10) + 2)".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((16) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x10) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).x =
            (((72340172838076673_u64 as u64).wrapping_mul(((((16) + (3)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((16) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x10) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((16) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x10) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            78_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).in_.a as *mut i16) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, in_) as usize)
                .wrapping_add((::std::mem::offset_of!(In, a) as usize))) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i16>(), ((32) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).in_.a as i32)
                    == ((((72340172838076673_u64 as u64).wrapping_mul((((32) as u8) as u64)))
                        as i16) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->in.a == REP(short, (0x20))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i16>(), ((32) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(short), (0x20))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).in_.a =
            (((72340172838076673_u64 as u64).wrapping_mul(((((32) + (1)) as u8) as u64))) as i16);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((32) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i16>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(short), (0x20) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((32) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i16>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(short), (0x20) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((32) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).in_.a as i32)
                    == ((((72340172838076673_u64 as u64)
                        .wrapping_mul(((((32) + (2)) as u8) as u64)))
                        as i16) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->in.a == REP(short, (0x20) + 2)".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((32) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i16>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(short), (0x20) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).in_.a =
            (((72340172838076673_u64 as u64).wrapping_mul(((((32) + (3)) as u8) as u64))) as i16);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((32) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i16>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(short), (0x20) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((32) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i16>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(short), (0x20) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            79_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).in_.b as *mut i32) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, in_) as usize)
                .wrapping_add((::std::mem::offset_of!(In, b) as usize))) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), ((48) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).in_.b)
                    == (((72340172838076673_u64 as u64).wrapping_mul((((48) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->in.b == REP(int, (0x30))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), ((48) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x30))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).in_.b =
            (((72340172838076673_u64 as u64).wrapping_mul(((((48) + (1)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((48) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x30) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((48) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x30) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((48) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).in_.b)
                    == (((72340172838076673_u64 as u64).wrapping_mul(((((48) + (2)) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->in.b == REP(int, (0x30) + 2)".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((48) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x30) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).in_.b =
            (((72340172838076673_u64 as u64).wrapping_mul(((((48) + (3)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((48) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x30) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((48) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x30) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            80_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(0) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(0_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), ((64) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(0) as usize] as i32)
                    == ((((72340172838076673_u64 as u64).wrapping_mul((((64) as u8) as u64))) as u8)
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[0] == REP(unsigned char, (0x40))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), ((64) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x40))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((64) + (1)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((64) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x40) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((64) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x40) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((64) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(0) as usize] as i32)
                    == ((((72340172838076673_u64 as u64)
                        .wrapping_mul(((((64) + (2)) as u8) as u64)))
                        as u8) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[0] == REP(unsigned char, (0x40) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((64) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x40) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((64) + (3)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((64) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x40) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((64) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x40) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            81_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(1) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(1_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), ((80) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(1) as usize] as i32)
                    == ((((72340172838076673_u64 as u64).wrapping_mul((((80) as u8) as u64))) as u8)
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[1] == REP(unsigned char, (0x50))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), ((80) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x50))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((80) + (1)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((80) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x50) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((80) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x50) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((80) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(1) as usize] as i32)
                    == ((((72340172838076673_u64 as u64)
                        .wrapping_mul(((((80) + (2)) as u8) as u64)))
                        as u8) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[1] == REP(unsigned char, (0x50) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((80) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x50) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((80) + (3)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((80) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x50) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((80) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x50) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            82_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(2) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(2_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), ((96) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(2) as usize] as i32)
                    == ((((72340172838076673_u64 as u64).wrapping_mul((((96) as u8) as u64))) as u8)
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[2] == REP(unsigned char, (0x60))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), ((96) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x60))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((96) + (1)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((96) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x60) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((96) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x60) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((96) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(2) as usize] as i32)
                    == ((((72340172838076673_u64 as u64)
                        .wrapping_mul(((((96) + (2)) as u8) as u64)))
                        as u8) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[2] == REP(unsigned char, (0x60) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((96) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x60) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((96) + (3)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((96) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x60) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((96) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x60) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            83_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(3) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(3_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), ((112) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(3) as usize] as i32)
                    == ((((72340172838076673_u64 as u64).wrapping_mul((((112) as u8) as u64)))
                        as u8) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[3] == REP(unsigned char, (0x70))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), ((112) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x70))"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(3) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((112) + (1)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((112) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x70) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((112) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x70) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((112) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).bytes[(3) as usize] as i32)
                    == ((((72340172838076673_u64 as u64)
                        .wrapping_mul(((((112) + (2)) as u8) as u64)))
                        as u8) as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->bytes[3] == REP(unsigned char, (0x70) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((112) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x70) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).bytes[(3) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((112) + (3)) as u8) as u64))) as u8);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((112) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(unsigned char), (0x70) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((112) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<u8>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(unsigned char), (0x70) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            84_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).arr[(0) as usize] as *mut i32) as *mut i32 as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, arr) as usize).wrapping_add(
                ((0_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), ((128) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).arr[(0) as usize])
                    == (((72340172838076673_u64 as u64).wrapping_mul((((128) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->arr[0] == REP(int, (0x80))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), ((128) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x80))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).arr[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((128) + (1)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((128) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x80) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((128) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x80) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((128) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).arr[(0) as usize])
                    == (((72340172838076673_u64 as u64)
                        .wrapping_mul(((((128) + (2)) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->arr[0] == REP(int, (0x80) + 2)".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((128) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x80) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).arr[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((128) + (3)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((128) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x80) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((128) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x80) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            85_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).arr[(1) as usize] as *mut i32) as *mut i32 as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, arr) as usize).wrapping_add(
                ((1_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), ((144) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).arr[(1) as usize])
                    == (((72340172838076673_u64 as u64).wrapping_mul((((144) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->arr[1] == REP(int, (0x90))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), ((144) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x90))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).arr[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((144) + (1)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((144) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x90) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((144) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x90) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((144) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).arr[(1) as usize])
                    == (((72340172838076673_u64 as u64)
                        .wrapping_mul(((((144) + (2)) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->arr[1] == REP(int, (0x90) + 2)".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((144) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x90) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).arr[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((144) + (3)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((144) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x90) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((144) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x90) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            86_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).arr[(2) as usize] as *mut i32) as *mut i32 as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, arr) as usize).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), ((8) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).arr[(2) as usize])
                    == (((72340172838076673_u64 as u64).wrapping_mul((((8) as u8) as u64))) as i32))
                    as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->arr[2] == REP(int, (0x08))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), ((8) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x08))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).arr[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((8) + (1)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((8) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x08) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((8) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x08) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((8) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).arr[(2) as usize])
                    == (((72340172838076673_u64 as u64).wrapping_mul(((((8) + (2)) as u8) as u64)))
                        as i32)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->arr[2] == REP(int, (0x08) + 2)".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((8) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x08) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).arr[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((8) + (3)) as u8) as u64))) as i32);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((8) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(int), (0x08) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((8) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i32>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(int), (0x08) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            87_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).tail as *mut i64) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset((::std::mem::offset_of!(S, tail)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_1(_p, ::std::mem::size_of::<i64>(), ((24) as u8))
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).tail)
                    == (((72340172838076673_u64 as u64).wrapping_mul((((24) as u8) as u64)))
                        as i64)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->tail == REP(long, (0x18))".as_ptr().cast_mut()).cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    all_bytes_2(_p, ::std::mem::size_of::<i64>(), ((24) as u8))
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(long), (0x18))".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).tail =
            (((72340172838076673_u64 as u64).wrapping_mul(((((24) + (1)) as u8) as u64))) as i64);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((24) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i64>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(long), (0x18) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((24) + (1)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i64>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(long), (0x18) + 1)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((24) + (2)) as u8);
            set_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
        });
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((((*(s)).tail)
                    == (((72340172838076673_u64 as u64).wrapping_mul(((((24) + (2)) as u8) as u64)))
                        as i64)) as i32)
                    != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"(s)->tail == REP(long, (0x18) + 2)".as_ptr().cast_mut())
                                .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((24) + (2)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i64>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(long), (0x18) + 2)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        (*(s)).tail =
            (((72340172838076673_u64 as u64).wrapping_mul(((((24) + (3)) as u8) as u64))) as i64);
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (sb).cast_const();
                    let _v: u8 = (((24) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i64>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(sb, sizeof(long), (0x18) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((unsafe {
                    let _p: *const u8 = (fb).cast_const();
                    let _v: u8 = (((24) + (3)) as u8);
                    all_bytes_2(_p, ::std::mem::size_of::<i64>(), _v)
                }) != 0)
                {
                } else {
                    (unsafe {
                        __assert_fail_4(
                            (c"all_bytes(fb, sizeof(long), (0x18) + 3)"
                                .as_ptr()
                                .cast_mut())
                            .cast_const(),
                            (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                            88_u32,
                            c"void check_struct(struct S *)".as_ptr(),
                        )
                    });
                }
            }
        };
    }
    (unsafe { check_final_3(s) });
    (unsafe {
        set_bytes_1(
            ((&mut (*s).in_ as *mut In) as *mut u8),
            ::std::mem::size_of::<In>(),
            33_u8,
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*s).in_.a as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((33) as u8) as u64))) as i16)
                    as i32)) as i32)
                != 0)
                && (((((*s).in_.b)
                    == (((72340172838076673_u64 as u64).wrapping_mul((((33) as u8) as u64)))
                        as i32)) as i32)
                    != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->in.a == REP(short, 0x21) && s->in.b == REP(int, 0x21)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        93_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut in_: In = In {
        a: (((72340172838076673_u64 as u64).wrapping_mul((((35) as u8) as u64))) as i16),
        b: (((72340172838076673_u64 as u64).wrapping_mul((((51) as u8) as u64))) as i32),
    };
    (*s).in_ = in_;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((unsafe {
                all_bytes_2(
                    ((&mut (*s).in_.a as *mut i16) as *mut u8).cast_const(),
                    ::std::mem::size_of::<i16>(),
                    35_u8,
                )
            }) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"all_bytes((unsigned char *)&s->in.a, sizeof(short), 0x23)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        96_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((unsafe {
                all_bytes_2(
                    ((&mut (*s).in_.b as *mut i32) as *mut u8).cast_const(),
                    ::std::mem::size_of::<i32>(),
                    51_u8,
                )
            }) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"all_bytes((unsigned char *)&s->in.b, sizeof(int), 0x33)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        97_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        let _p: *mut u8 = ((*s).arr.as_mut_ptr() as *mut u8);
        let _n: usize = ::std::mem::size_of::<[i32; 3]>();
        set_bytes_1(_p, _n, 133_u8)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*s).arr[(0) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((133) as u8) as u64))) as i32))
                as i32)
                != 0)
                && (((((*s).arr[(2) as usize])
                    == (((72340172838076673_u64 as u64).wrapping_mul((((133) as u8) as u64)))
                        as i32)) as i32)
                    != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->arr[0] == REP(int, 0x85) && s->arr[2] == REP(int, 0x85)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        102_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        let _p: *mut u8 = ((&mut (*s).arr as *mut [i32; 3]) as *mut u8);
        let _n: usize = ::std::mem::size_of::<[i32; 3]>();
        set_bytes_1(_p, _n, 134_u8)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).arr[(1) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((134) as u8) as u64))) as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->arr[1] == REP(int, 0x86)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        104_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    (*s).arr[(0) as usize] =
        (((72340172838076673_u64 as u64).wrapping_mul((((131) as u8) as u64))) as i32);
    (*s).arr[(1) as usize] =
        (((72340172838076673_u64 as u64).wrapping_mul((((147) as u8) as u64))) as i32);
    (*s).arr[(2) as usize] =
        (((72340172838076673_u64 as u64).wrapping_mul((((11) as u8) as u64))) as i32);
    let mut ib: *mut i32 = ((*s).bytes.as_mut_ptr() as *mut i32);
    (*ib) = (((72340172838076673_u64 as u64).wrapping_mul((((69) as u8) as u64))) as i32);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((unsafe {
                let _p: *const u8 = ((*s).bytes.as_mut_ptr()).cast_const();
                let _n: usize = ::std::mem::size_of::<[u8; 4]>();
                all_bytes_2(_p, _n, 69_u8)
            }) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"all_bytes(s->bytes, sizeof(s->bytes), 0x45)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        110_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    (*s).bytes[(0) as usize] = 67_u8;
    (*s).bytes[(1) as usize] = 83_u8;
    (*s).bytes[(2) as usize] = 99_u8;
    (*s).bytes[(3) as usize] = 115_u8;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((*ib) == (*(((&mut (*s).bytes as *mut [u8; 4]) as *mut u8) as *mut i32))) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"*ib == *(int *)(unsigned char *)&s->bytes"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        115_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut v: *mut ::libc::c_void = ((&mut (*s).in_.b as *mut i32) as *mut ::libc::c_void);
    (*(v as *mut i32)) = 11;
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).in_.b) == (11)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->in.b == 11".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        120_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut twelve: i32 = 12;
    (unsafe {
        memcpy_6(
            ((&mut (*s).x as *mut i32) as *mut ::libc::c_void),
            ((&mut twelve as *mut i32) as *const ::libc::c_void),
            ::std::mem::size_of::<i32>(),
        )
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).x) == (12)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"s->x == 12".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        123_u32,
                        c"void check_struct(struct S *)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        memset_7(
            ((&mut (*s).in_.b as *mut i32) as *mut ::libc::c_void),
            51,
            ::std::mem::size_of::<i32>(),
        )
    });
    (unsafe {
        memset_7(
            ((&mut (*s).x as *mut i32) as *mut ::libc::c_void),
            19,
            ::std::mem::size_of::<i32>(),
        )
    });
    (unsafe { check_final_3(s) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: S = <S>::default();
    (unsafe {
        let ___s: *mut ::libc::c_void = ((&mut local as *mut S) as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<S>();
        memset_7(___s, 0, ___n)
    });
    (unsafe { check_struct_5((&mut local as *mut S)) });
    let mut arr: [S; 2] = [<S>::default(); 2];
    (unsafe {
        let ___s: *mut ::libc::c_void = (arr.as_mut_ptr() as *mut ::libc::c_void);
        let ___n: usize = ::std::mem::size_of::<[S; 2]>();
        memset_7(___s, 0, ___n)
    });
    (unsafe { check_struct_5((&mut arr[(1) as usize] as *mut S)) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((unsafe {
                all_bytes_2(
                    ((&mut arr[(0) as usize] as *mut S) as *mut S as *mut u8).cast_const(),
                    ::std::mem::size_of::<S>(),
                    0_u8,
                )
            }) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"all_bytes((unsigned char *)&arr[0], sizeof(struct S), 0)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        139_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut heap: *mut S = ((unsafe { calloc_8(1_usize, ::std::mem::size_of::<S>()) }) as *mut S);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((heap).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"heap != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        143_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { check_struct_5(heap) });
    (unsafe { free_9((heap as *mut ::libc::c_void)) });
    let mut raw_: *mut u8 = ((unsafe { malloc_10(::std::mem::size_of::<S>()) }) as *mut u8);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((raw_).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"raw != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        149_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { memset_7((raw_ as *mut ::libc::c_void), 0, ::std::mem::size_of::<S>()) });
    let mut view: *mut S = (raw_ as *mut S);
    (unsafe { check_struct_5(view) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*raw_.offset((::std::mem::offset_of!(S, tail)) as isize)) as i32) == (27))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"raw[offsetof(struct S, tail)] == 0x1b".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        153_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        let _p: *mut u8 = raw_.offset((::std::mem::offset_of!(S, arr)) as isize);
        set_bytes_1(_p, ::std::mem::size_of::<i32>(), 119_u8)
    });
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*view).arr[(0) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((119) as u8) as u64))) as i32))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"view->arr[0] == REP(int, 0x77)".as_ptr().cast_mut()).cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        155_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (*view).bytes[(3) as usize] = 17_u8;
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*raw_.offset(
                ((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(3_usize)) as isize,
            )) as i32)
                == (17)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"raw[offsetof(struct S, bytes) + 3] == 0x11"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"field_reinterpret_write_through.c".as_ptr().cast_mut()).cast_const(),
                        157_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { free_9((raw_ as *mut ::libc::c_void)) });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn memcpy_6(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn memset_7(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn calloc_8(mut __nmemb: usize, mut __size: usize) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn free_9(mut __ptr: *mut ::libc::c_void) {
    unimplemented!()
}
pub unsafe fn malloc_10(mut __size: usize) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
