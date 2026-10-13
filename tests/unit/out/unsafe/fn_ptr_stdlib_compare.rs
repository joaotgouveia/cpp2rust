extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_0,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_0 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
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
pub struct _G_fpos_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos64_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut ::libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::libc::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option<unsafe fn(*mut ::libc::c_void, *mut libc::c_char, usize) -> i64>,
    pub write: Option<unsafe fn(*mut ::libc::c_void, *const libc::c_char, usize) -> i64>,
    pub seek: Option<unsafe fn(*mut ::libc::c_void, *mut i64, i32) -> i32>,
    pub close: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
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
    pub __value32: anon_1,
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
pub struct anon_1 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_1 {
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
pub unsafe fn my_alternative_fread_2(
    mut p: *mut libc::c_char,
    mut n: usize,
    mut m: usize,
    mut f: *mut ::libc::c_void,
) -> usize {
    return 22_usize;
}
pub unsafe fn my_alternative_fwrite_3(
    mut p: *const libc::c_char,
    mut n: usize,
    mut m: usize,
    mut f: *mut ::libc::c_void,
) -> usize {
    return 33_usize;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn1: Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize> =
        Some(fread_4);
    (if ((fn1) == (Some(fread_4))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"fn1 == fread".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !((fn1).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"fn1 != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut fn2: Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize>,
            Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
        >(Some(fread_4));
    (if ((fn1)
        == (std::mem::transmute::<
            Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize>,
        >(fn2)))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"fn1 == (fread_t)fn2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut f3: Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize>,
        >(Some(my_alternative_fread_2));
    (if ((unsafe { (f3).unwrap()(std::ptr::null_mut(), 0_usize, 0_usize, std::ptr::null_mut()) })
        == (22_usize))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"(*f3)(nullptr, 0, 0, nullptr) == 22".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut _IO_FILE = (unsafe { fopen_6(c"/dev/zero".as_ptr(), c"rb".as_ptr()) });
        (if !((stream).is_null()) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"stream != nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
        (unsafe {
            let ___s: *mut ::libc::c_void = (buf.as_mut_ptr() as *mut ::libc::c_void);
            let ___n: usize = ::std::mem::size_of::<[libc::c_char; 16]>();
            memset_7(___s, (('X' as libc::c_char) as i32), ___n)
        });
        let mut n: usize = (unsafe {
            (fread_4)(
                (buf.as_mut_ptr() as *mut ::libc::c_void),
                1_usize,
                10_usize,
                stream,
            )
        });
        (if ((n) == (10_usize)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"n == 10".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            (if ((buf[(i) as usize] as i32) == (0)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_5(
                        c"buf[i] == 0".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            (if ((buf[(i) as usize] as i32) == (('X' as libc::c_char) as i32)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_5(
                        c"buf[i] == \'X\'".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
            i.prefix_inc();
        }
        (unsafe { fclose_8(stream) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut _IO_FILE = (unsafe { fopen_6(c"/dev/zero".as_ptr(), c"rb".as_ptr()) });
        (if !((stream).is_null()) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"stream != nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
        (unsafe {
            let ___s: *mut ::libc::c_void = (buf.as_mut_ptr() as *mut ::libc::c_void);
            let ___n: usize = ::std::mem::size_of::<[libc::c_char; 16]>();
            memset_7(___s, (('X' as libc::c_char) as i32), ___n)
        });
        let mut n: usize = (unsafe {
            (fn1).unwrap()(
                (buf.as_mut_ptr() as *mut ::libc::c_void),
                1_usize,
                10_usize,
                stream,
            )
        });
        (if ((n) == (10_usize)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"n == 10".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            (if ((buf[(i) as usize] as i32) == (0)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_5(
                        c"buf[i] == 0".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            (if ((buf[(i) as usize] as i32) == (('X' as libc::c_char) as i32)) {
                &(0);
            } else {
                (unsafe {
                    __assert_fail_5(
                        c"buf[i] == \'X\'".as_ptr(),
                        concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                        line!() as u32,
                        c"int main()".as_ptr(),
                    )
                });
            });
            i.prefix_inc();
        }
        (unsafe { fclose_8(stream) });
    }
    let mut gn1: Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize> =
        Some(fwrite_9);
    (if ((gn1) == (Some(fwrite_9))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"gn1 == fwrite".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !((gn1).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"gn1 != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut gn2: Option<
        unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize,
    > = std::mem::transmute::<
        Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize>,
        Option<unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
    >(Some(fwrite_9));
    (if ((gn1)
        == (std::mem::transmute::<
            Option<unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize>,
        >(gn2)))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"gn1 == (fwrite_t)gn2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut g3: Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut _IO_FILE) -> usize>,
        >(Some(my_alternative_fwrite_3));
    (if ((unsafe { (g3).unwrap()(std::ptr::null(), 0_usize, 0_usize, std::ptr::null_mut()) })
        == (33_usize))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"(*g3)(nullptr, 0, 0, nullptr) == 33".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut _IO_FILE = (unsafe { fopen_6(c"/dev/null".as_ptr(), c"wb".as_ptr()) });
        (if !((stream).is_null()) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"stream != nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        let mut buf: [libc::c_char; 10] = [(0 as libc::c_char); 10];
        (unsafe {
            let ___s: *mut ::libc::c_void = (buf.as_mut_ptr() as *mut ::libc::c_void);
            let ___n: usize = ::std::mem::size_of::<[libc::c_char; 10]>();
            memset_7(___s, (('Y' as libc::c_char) as i32), ___n)
        });
        let mut n: usize = (unsafe {
            (fwrite_9)(
                (buf.as_mut_ptr() as *const ::libc::c_void),
                1_usize,
                10_usize,
                stream,
            )
        });
        (if ((n) == (10_usize)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"n == 10".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        (unsafe { fclose_8(stream) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut _IO_FILE = (unsafe { fopen_6(c"/dev/null".as_ptr(), c"wb".as_ptr()) });
        (if !((stream).is_null()) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"stream != nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        let mut buf: [libc::c_char; 10] = [(0 as libc::c_char); 10];
        (unsafe {
            let ___s: *mut ::libc::c_void = (buf.as_mut_ptr() as *mut ::libc::c_void);
            let ___n: usize = ::std::mem::size_of::<[libc::c_char; 10]>();
            memset_7(___s, (('Y' as libc::c_char) as i32), ___n)
        });
        let mut n: usize = (unsafe {
            (gn1).unwrap()(
                (buf.as_mut_ptr() as *const ::libc::c_void),
                1_usize,
                10_usize,
                stream,
            )
        });
        (if ((n) == (10_usize)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_5(
                    c"n == 10".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        (unsafe { fclose_8(stream) });
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn fread_4(
    mut __ptr: *mut ::libc::c_void,
    mut __size: usize,
    mut __n: usize,
    mut __stream: *mut _IO_FILE,
) -> usize {
    unimplemented!()
}
pub unsafe fn __assert_fail_5(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn fopen_6(
    mut __filename: *const libc::c_char,
    mut __modes: *const libc::c_char,
) -> *mut _IO_FILE {
    unimplemented!()
}
pub unsafe fn memset_7(
    mut __s: *mut ::libc::c_void,
    mut __c: i32,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn fclose_8(mut __stream: *mut _IO_FILE) -> i32 {
    unimplemented!()
}
pub unsafe fn fwrite_9(
    mut __ptr: *const ::libc::c_void,
    mut __size: usize,
    mut __n: usize,
    mut __s: *mut _IO_FILE,
) -> usize {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
