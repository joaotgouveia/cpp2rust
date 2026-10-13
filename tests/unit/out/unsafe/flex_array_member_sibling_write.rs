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
pub struct S {
    pub n: i32,
    pub name: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E {
    pub id: i32,
    pub w: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct T {
    pub n: i32,
    pub cap: i32,
    pub a: [E; 1],
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: *mut S = ((unsafe {
        calloc_1(
            1_usize,
            (::std::mem::size_of::<S>() as usize).wrapping_add(8_usize),
        )
    }) as *mut S);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((s).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"s != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        23_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe {
        memcpy_3(
            ((*s).name.as_mut_ptr() as *mut ::libc::c_void),
            (c"abcdefg".as_ptr().cast_mut() as *const ::libc::c_void),
            8_usize,
        )
    });
    (*s).n = 5;
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*s).n) == (5)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"s->n == 5".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_4(
                    ((*s).name.as_mut_ptr()).cast_const(),
                    (c"abcdefg".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"strcmp(s->name, \"abcdefg\") == 0".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        27_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { free_5((s as *mut ::libc::c_void)) });
    let mut t: *mut T = ((unsafe {
        malloc_6(
            (::std::mem::size_of::<T>() as usize)
                .wrapping_add((::std::mem::size_of::<E>() as usize)),
        )
    }) as *mut T);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((t).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"t != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        31_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (*t).n = 2;
    (*t).cap = 2;
    (*(*t).a.as_mut_ptr().add((0) as usize)).id = 10;
    (*(*t).a.as_mut_ptr().add((1) as usize)).w = 20;
    (*t).n = 3;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*(*t).a.as_mut_ptr().add((0) as usize)).id) == (10)) as i32) != 0)
                && (((((*(*t).a.as_mut_ptr().add((1) as usize)).w) == (20)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"t->a[0].id == 10 && t->a[1].w == 20".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        37_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut tail: *mut E = ((&mut (*t.offset((1) as isize)) as *mut T) as *mut T as *mut E);
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((tail) == ((*t).a.as_mut_ptr().add((1) as usize))) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"tail == &t->a[1]".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        39_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (*tail.offset((0) as isize)).id = 30;
    (*t).cap = 4;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*(*t).a.as_mut_ptr().add((1) as usize)).id) == (30)) as i32) != 0)
                && (((((*(*t).a.as_mut_ptr().add((1) as usize)).w) == (20)) as i32) != 0))
                as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"t->a[1].id == 30 && t->a[1].w == 20".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        42_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((*t).n) == (3)) as i32) != 0) && (((((*t).cap) == (4)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_2(
                        (c"t->n == 3 && t->cap == 4".as_ptr().cast_mut()).cast_const(),
                        (c"flex_array_member_sibling_write.c".as_ptr().cast_mut()).cast_const(),
                        43_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { free_5((t as *mut ::libc::c_void)) });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn calloc_1(mut __nmemb: usize, mut __size: usize) -> *mut ::libc::c_void {
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
pub unsafe fn memcpy_3(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn strcmp_4(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn free_5(mut __ptr: *mut ::libc::c_void) {
    unimplemented!()
}
pub unsafe fn malloc_6(mut __size: usize) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
