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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_ptr__long_ptr_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_double_ptr__long_double_ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S_ptr__S_ptr_ {}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S4_ptr__S4_ptr_ {}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S8_ptr__S8_ptr_ {}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S16_ptr__S16_ptr_ {}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S32_ptr__S32_ptr_ {}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S64_ptr__S64_ptr_ {}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_S128_ptr__S128_ptr_ {}
pub unsafe fn check_12(mut p: *mut libc::c_char) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<libc::c_char>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}
pub unsafe fn check_13(mut p: *mut i32) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<i32>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
    {}
}
pub unsafe fn check_14(mut p: *mut i64) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<i64>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
}
pub unsafe fn check_15(mut p: *mut f64) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<f64>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
}
pub unsafe fn check_16(mut p: *mut f64) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<f64>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
}
pub unsafe fn check_17(mut p: *mut S) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}
pub unsafe fn check_18(mut p: *mut S4) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S4>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
    {}
}
pub unsafe fn check_19(mut p: *mut S8) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S8>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
    {}
}
pub unsafe fn check_20(mut p: *mut S16) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S16>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
    {}
}
pub unsafe fn check_21(mut p: *mut S32) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S32>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
    {}
}
pub unsafe fn check_22(mut p: *mut S64) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S64>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {}
}
pub unsafe fn check_23(mut p: *mut S128) {
    const _: () = assert!(true, "");;
    let alignment: usize = ::std::mem::align_of::<S128>();
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
    {
        assert!(((p) == (unimplemented!())));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
#[repr(C, align(4))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S4 {}
#[repr(C, align(8))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S8 {}
#[repr(C, align(16))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S16 {}
#[repr(C, align(32))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S32 {}
#[repr(C, align(64))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S64 {}
#[repr(C, align(128))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S128 {}
pub unsafe fn tests_24() -> bool {
    let mut c: libc::c_char = (0 as libc::c_char);
    let mut i: i32 = 0_i32;
    let mut l: i64 = 0_i64;
    let mut d: f64 = 0_f64;
    let mut ld: f64 = 0_f64;
    (unsafe { check_12((&mut c as *mut libc::c_char)) });
    (unsafe { check_13((&mut i as *mut i32)) });
    (unsafe { check_14((&mut l as *mut i64)) });
    (unsafe { check_15((&mut d as *mut f64)) });
    (unsafe { check_16((&mut ld as *mut f64)) });
    let mut s: S = <S>::default();
    let mut s4: S4 = <S4>::default();
    let mut s8: S8 = <S8>::default();
    let mut s16: S16 = <S16>::default();
    let mut s32: S32 = <S32>::default();
    let mut s64: S64 = <S64>::default();
    let mut s128: S128 = <S128>::default();
    (unsafe { check_17((&mut s as *mut S)) });
    (unsafe { check_18((&mut s4 as *mut S4)) });
    (unsafe { check_19((&mut s8 as *mut S8)) });
    (unsafe { check_20((&mut s16 as *mut S16)) });
    (unsafe { check_21((&mut s32 as *mut S32)) });
    (unsafe { check_22((&mut s64 as *mut S64)) });
    (unsafe { check_23((&mut s128 as *mut S128)) });
    return true;
}
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
    (unsafe { tests_24() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
