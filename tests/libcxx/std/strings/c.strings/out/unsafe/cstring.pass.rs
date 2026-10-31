extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_void_ptr__void_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_void_ptr__const_void_ptr_ {}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut s: usize = 0_usize;
        let mut vp: *mut ::libc::c_void = std::ptr::null_mut();
        let mut vpc: *const ::libc::c_void = std::ptr::null();
        let mut cp: *mut libc::c_char = std::ptr::null_mut();
        let mut cpc: *const libc::c_char = std::ptr::null();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const libc::c_char = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const libc::c_char =
            (libc::strchr(s, (('l' as libc::c_char) as i32)) as *const libc::c_char);
        assert!(((res) == (&(*s.offset((2) as isize)) as *const libc::c_char)));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut libc::c_char = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut libc::c_char = unimplemented!();
        assert!(((res) == (&mut (*s.offset((2) as isize)) as *mut libc::c_char)));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const libc::c_char = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const libc::c_char =
            (libc::strpbrk(s, c"el".as_ptr()) as *const libc::c_char);
        assert!(((res) == (&(*s.offset((1) as isize)) as *const libc::c_char)));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut libc::c_char = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut libc::c_char = libc::strpbrk(s, c"el".as_ptr());
        assert!(((res) == (&mut (*s.offset((1) as isize)) as *mut libc::c_char)));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const libc::c_char = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const libc::c_char =
            (libc::strrchr(s, (('l' as libc::c_char) as i32)) as *const libc::c_char);
        assert!(((res) == (&(*s.offset((9) as isize)) as *const libc::c_char)));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut libc::c_char = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut libc::c_char = libc::strrchr(s, (('l' as libc::c_char) as i32));
        assert!(((res) == (&mut (*s.offset((9) as isize)) as *mut libc::c_char)));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut count: usize = 11_usize;
        let mut s: *const ::libc::c_void = (storage.as_mut_ptr() as *const ::libc::c_void);
        const _: () = assert!(true, "");;
        let mut res: *const ::libc::c_void = libc::memchr(
            s as *const ::libc::c_void,
            (('l' as libc::c_char) as i32),
            count as usize,
        ) as *const ::libc::c_void;
        assert!(
            ((res) == ((&mut storage[(2) as usize] as *mut libc::c_char) as *const ::libc::c_void))
        );
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut count: usize = 11_usize;
        let mut s: *mut ::libc::c_void = (storage.as_mut_ptr() as *mut ::libc::c_void);
        const _: () = assert!(true, "");;
        let mut res: *mut ::libc::c_void = unimplemented!();
        assert!(
            ((res) == ((&mut storage[(2) as usize] as *mut libc::c_char) as *mut ::libc::c_void))
        );
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const libc::c_char = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const libc::c_char =
            (libc::strstr(s, c"wor".as_ptr()) as *const libc::c_char);
        assert!(((res) == ((&mut storage[(6) as usize] as *mut libc::c_char).cast_const())));
    }
    {
        let mut storage: [libc::c_char; 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut libc::c_char = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut libc::c_char = libc::strstr(s, c"wor".as_ptr());
        assert!(((res) == (&mut storage[(6) as usize] as *mut libc::c_char)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
