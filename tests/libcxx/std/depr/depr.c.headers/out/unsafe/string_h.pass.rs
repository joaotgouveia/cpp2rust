extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_void_ptr__void_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_5 = ();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut s: () = 0_();
        let mut vp: () = unimplemented!();
        let mut vpc: () = unimplemented!();
        let mut cp: *mut () = std::ptr::null_mut();
        let mut cpc: *const () = std::ptr::null();
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
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const () = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut () = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const () = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut () = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const () = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut () = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut count: () = 11_();
        let mut s: () = (storage.as_mut_ptr() as ());
        const _: () = assert!(true, "");;
        let mut res: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut count: () = 11_();
        let mut s: () = (storage.as_mut_ptr() as ());
        const _: () = assert!(true, "");;
        let mut res: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *const () = (storage.as_mut_ptr()).cast_const();
        const _: () = assert!(true, "");;
        let mut res: *const () = unimplemented!();
        unimplemented!();
    }
    {
        let mut storage: [(); 12] = std::mem::transmute(*b"hello world\0");
        let mut s: *mut () = storage.as_mut_ptr();
        const _: () = assert!(true, "");;
        let mut res: *mut () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
