extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_memcpy_0() {
    let src: [(); 6] = std::mem::transmute(*b"hello\0");
    let mut dst: [(); 6] = [
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memset_1() {
    let mut buf: [(); 4] = [unimplemented!(); 4];
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memcmp_2() {
    let a: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let b: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let c: [(); 4] = [1_(), 2_(), 9_(), 4_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memmove_3() {
    let mut buf: [(); 6] = [
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('d' as ()),
        ('e' as ()),
        ('\0' as ()),
    ];
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strchr_4() {
    let mut s: *const () = c"hello world".as_ptr();
    let mut r: *const () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strlen_5() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strcmp_6() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = c"abc".as_ptr();
    let mut q: *const () = c"abd".as_ptr();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strncmp_7() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = c"abcdef".as_ptr();
    let mut q: *const () = c"abcxyz".as_ptr();
    let mut buf: [(); 7] = [
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('d' as ()),
        ('e' as ()),
        ('f' as ()),
        ('\0' as ()),
    ];
    let mut n: () = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memchr_8() {
    let data: [(); 4] = [16_(), 32_(), 48_(), 64_()];
    let mut r: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: () = (data.as_ptr() as ());
    let mut n: () = 4_();
    unimplemented!();
}
pub unsafe fn test_strrchr_9() {
    let mut s: *const () = c"hello world".as_ptr();
    let mut r: *const () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('a' as ()), ('\0' as ())];
    unimplemented!();
}
pub unsafe fn test_strdup_10() {
    let mut d: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = c"world".as_ptr();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    let mut d2: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d3: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strcspn_11() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: *const () = c"hello".as_ptr();
    let mut rej: *const () = c"el".as_ptr();
    unimplemented!();
}
pub unsafe fn test_strspn_12() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: *const () = c"hello".as_ptr();
    let mut acc: *const () = c"hel".as_ptr();
    unimplemented!();
}
pub unsafe fn test_strstr_13() {
    let mut h: *const () = c"hello world".as_ptr();
    let mut r: *const () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 6] = [
        ('h' as ()),
        ('e' as ()),
        ('l' as ()),
        ('l' as ()),
        ('o' as ()),
        ('\0' as ()),
    ];
    unimplemented!();
}
pub unsafe fn test_strpbrk_14() {
    let mut s: *const () = c"hello world".as_ptr();
    let mut r: *const () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    unimplemented!();
}
pub unsafe fn test_strcasecmp_15() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = c"FOO".as_ptr();
    let mut q: *const () = c"foo".as_ptr();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_memcpy_0() });
    (unsafe { test_memset_1() });
    (unsafe { test_memcmp_2() });
    (unsafe { test_memmove_3() });
    (unsafe { test_strchr_4() });
    (unsafe { test_strlen_5() });
    (unsafe { test_strcmp_6() });
    (unsafe { test_strncmp_7() });
    (unsafe { test_memchr_8() });
    (unsafe { test_strrchr_9() });
    (unsafe { test_strdup_10() });
    (unsafe { test_strcspn_11() });
    (unsafe { test_strspn_12() });
    (unsafe { test_strstr_13() });
    (unsafe { test_strpbrk_14() });
    (unsafe { test_strcasecmp_15() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
