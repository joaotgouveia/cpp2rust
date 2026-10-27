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
pub struct test_macros_detail_is_same_const_bool__const_bool_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {}
impl D {
    pub unsafe fn allocator_type() -> () {
        return 0_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E {}
pub unsafe fn test_2() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_3() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_4() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_5() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_6() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_7() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_8() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
}
pub unsafe fn test_9() {
    const _: () = assert!(true, "(std::uses_allocator<T, A>::value == Expected)");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<bool, Expected>, std::uses_allocator<T, A> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "(std::uses_allocator_v<T, A> == Expected)");;
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
