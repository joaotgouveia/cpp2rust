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
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut r: () = (unsafe { memcpy_1((dst.as_mut_ptr() as ()), (src.as_ptr() as ()), 6_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memset_2() {
    let mut buf: [(); 4] = [unimplemented!(); 4];
    let mut r: () = (unsafe { memset_3((buf.as_mut_ptr() as ()), ('x' as ()), 4_()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memcmp_4() {
    let a: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let b: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let c: [(); 4] = [1_(), 2_(), 9_(), 4_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_memmove_5() {
    let mut buf: [(); 6] = [
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('d' as ()),
        ('e' as ()),
        ('\0' as ()),
    ];
    let mut r: () = (unsafe {
        let ___dest: () = (buf.as_mut_ptr().offset((1_()) as isize) as ());
        let ___src: () = (buf.as_mut_ptr() as ());
        memmove_6(___dest, ___src, 4_())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strchr_7() {
    let mut s: *const () = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut () = (unsafe { strchr_8((s as *mut ()).cast_const(), ('w' as ())) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strlen_9() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let buf: [(); 8] = std::mem::transmute(*b"one\0two\0");
    let mut first: *const () = buf.as_ptr();
    let mut second: *const () =
        (&buf[((unsafe { strlen_10(first) }).wrapping_add(1_())) as usize] as *const ());
    unimplemented!();
    let mut big : [ () ; 64] = std::mem::transmute(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") ;
    unimplemented!();
    big[(2_()) as usize] = ('x' as ());
    big[(3_()) as usize] = ('\0' as ());
    unimplemented!();
}
pub unsafe fn test_strcmp_11() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = (c"abc".as_ptr().cast_mut()).cast_const();
    let mut q: *const () = (c"abd".as_ptr().cast_mut()).cast_const();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_strncmp_12() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = (c"abcdef".as_ptr().cast_mut()).cast_const();
    let mut q: *const () = (c"abcxyz".as_ptr().cast_mut()).cast_const();
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
pub unsafe fn test_memchr_13() {
    let data: [(); 4] = [16_(), 32_(), 48_(), 64_()];
    let mut r: () = (unsafe { memchr_14((data.as_ptr() as ()), 48_(), 4_()) });
    unimplemented!();
    unimplemented!();
    let mut p: () = (data.as_ptr() as ());
    let mut n: () = 4_();
    unimplemented!();
}
pub unsafe fn test_strrchr_15() {
    let mut s: *const () = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut () = (unsafe { strrchr_16((s as *mut ()).cast_const(), ('l' as ())) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('a' as ()), ('\0' as ())];
    unimplemented!();
}
pub unsafe fn test_strcspn_17() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: *const () = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut rej: *const () = (c"el".as_ptr().cast_mut()).cast_const();
    unimplemented!();
}
pub unsafe fn test_strspn_18() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: *const () = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut acc: *const () = (c"hel".as_ptr().cast_mut()).cast_const();
    unimplemented!();
}
pub unsafe fn test_strstr_19() {
    let mut h: *const () = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut () = (unsafe {
        strstr_20(
            (h as *mut ()).cast_const(),
            (c"world".as_ptr().cast_mut()).cast_const(),
        )
    });
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
pub unsafe fn test_strpbrk_21() {
    let mut s: *const () = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut () = (unsafe {
        strpbrk_22(
            (s as *mut ()).cast_const(),
            (c"wo".as_ptr().cast_mut()).cast_const(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 4] = [('a' as ()), ('b' as ()), ('c' as ()), ('\0' as ())];
    unimplemented!();
}
pub unsafe fn test_strcasecmp_23() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *const () = (c"FOO".as_ptr().cast_mut()).cast_const();
    let mut q: *const () = (c"foo".as_ptr().cast_mut()).cast_const();
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
    (unsafe { test_memset_2() });
    (unsafe { test_memcmp_4() });
    (unsafe { test_memmove_5() });
    (unsafe { test_strchr_7() });
    (unsafe { test_strlen_9() });
    (unsafe { test_strcmp_11() });
    (unsafe { test_strncmp_12() });
    (unsafe { test_memchr_13() });
    (unsafe { test_strrchr_15() });
    (unsafe { test_strcspn_17() });
    (unsafe { test_strspn_18() });
    (unsafe { test_strstr_19() });
    (unsafe { test_strpbrk_21() });
    (unsafe { test_strcasecmp_23() });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
