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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut p: *mut i32 = ((unsafe { malloc_1(::std::mem::size_of::<i32>()) }) as *mut i32);
        (*p) = 42;
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*p) == (42)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"*p == 42".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe { free_3((p as *mut ::libc::c_void)) });
        let mut arr: *mut i32 =
            ((unsafe { malloc_1((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))) })
                as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            (*arr.offset((i) as isize)) = ((i) * (10));
            i.postfix_inc();
        }
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*arr.offset((0) as isize)) == (0)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"arr[0] == 0".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*arr.offset((3) as isize)) == (30)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"arr[3] == 30".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe { free_3((arr as *mut ::libc::c_void)) });
        let mut grow: *mut i32 =
            ((unsafe { malloc_1((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))) })
                as *mut i32);
        (*grow.offset((0) as isize)) = 1;
        (*grow.offset((1) as isize)) = 2;
        grow = ((unsafe {
            realloc_4(
                (grow as *mut ::libc::c_void),
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
        }) as *mut i32);
        (*grow.offset((2) as isize)) = 3;
        (*grow.offset((3) as isize)) = 4;
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((0) as isize)) == (1)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[0] == 1".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((1) as isize)) == (2)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[1] == 2".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((2) as isize)) == (3)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[2] == 3".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((3) as isize)) == (4)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[3] == 4".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            39_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe { free_3((grow as *mut ::libc::c_void)) });
        let mut zeros: *mut i32 =
            ((unsafe { calloc_5(4_usize, ::std::mem::size_of::<i32>()) }) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            {
                &(::std::mem::size_of::<i32>());
                {
                    if ((((*zeros.offset((i) as isize)) == (0)) as i32) != 0) {
                    } else {
                        (unsafe {
                            __assert_fail_2(
                                (c"zeros[i] == 0".as_ptr().cast_mut()).cast_const(),
                                (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                                39_u32,
                                c"int main()".as_ptr(),
                            )
                        });
                    }
                }
            };
            i.postfix_inc();
        }
        (unsafe { free_3((zeros as *mut ::libc::c_void)) });
    }
    let mut pmalloc: Option<unsafe fn(usize) -> *mut ::libc::c_void> = Some(malloc_1);
    let mut pfree: Option<unsafe fn(*mut ::libc::c_void)> = Some(free_3);
    let mut prealloc: Option<unsafe fn(*mut ::libc::c_void, usize) -> *mut ::libc::c_void> =
        Some(realloc_4);
    let mut pcalloc: Option<unsafe fn(usize, usize) -> *mut ::libc::c_void> = Some(calloc_5);
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut p: *mut i32 =
            ((unsafe { (pmalloc).unwrap()(::std::mem::size_of::<i32>()) }) as *mut i32);
        (*p) = 42;
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*p) == (42)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"*p == 42".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe { (pfree).unwrap()((p as *mut ::libc::c_void)) });
        let mut arr: *mut i32 = ((unsafe {
            (pmalloc).unwrap()((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
        }) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            (*arr.offset((i) as isize)) = ((i) * (10));
            i.postfix_inc();
        }
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*arr.offset((0) as isize)) == (0)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"arr[0] == 0".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*arr.offset((3) as isize)) == (30)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"arr[3] == 30".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe { (pfree).unwrap()((arr as *mut ::libc::c_void)) });
        let mut grow: *mut i32 = ((unsafe {
            (pmalloc).unwrap()((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
        }) as *mut i32);
        (*grow.offset((0) as isize)) = 1;
        (*grow.offset((1) as isize)) = 2;
        grow = ((unsafe {
            (prealloc).unwrap()(
                (grow as *mut ::libc::c_void),
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
        }) as *mut i32);
        (*grow.offset((2) as isize)) = 3;
        (*grow.offset((3) as isize)) = 4;
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((0) as isize)) == (1)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[0] == 1".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((1) as isize)) == (2)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[1] == 2".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((2) as isize)) == (3)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[2] == 3".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        {
            &(::std::mem::size_of::<i32>());
            {
                if ((((*grow.offset((3) as isize)) == (4)) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_2(
                            (c"grow[3] == 4".as_ptr().cast_mut()).cast_const(),
                            (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                            46_u32,
                            c"int main()".as_ptr(),
                        )
                    });
                }
            }
        };
        (unsafe { (pfree).unwrap()((grow as *mut ::libc::c_void)) });
        let mut zeros: *mut i32 =
            ((unsafe { (pcalloc).unwrap()(4_usize, ::std::mem::size_of::<i32>()) }) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            {
                &(::std::mem::size_of::<i32>());
                {
                    if ((((*zeros.offset((i) as isize)) == (0)) as i32) != 0) {
                    } else {
                        (unsafe {
                            __assert_fail_2(
                                (c"zeros[i] == 0".as_ptr().cast_mut()).cast_const(),
                                (c"malloc_realloc_free.c".as_ptr().cast_mut()).cast_const(),
                                46_u32,
                                c"int main()".as_ptr(),
                            )
                        });
                    }
                }
            };
            i.postfix_inc();
        }
        (unsafe { (pfree).unwrap()((zeros as *mut ::libc::c_void)) });
    }
    return 0;
}
pub unsafe fn malloc_1(mut __size: usize) -> *mut ::libc::c_void {
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
pub unsafe fn free_3(mut __ptr: *mut ::libc::c_void) {
    unimplemented!()
}
pub unsafe fn realloc_4(mut __ptr: *mut ::libc::c_void, mut __size: usize) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn calloc_5(mut __nmemb: usize, mut __size: usize) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
