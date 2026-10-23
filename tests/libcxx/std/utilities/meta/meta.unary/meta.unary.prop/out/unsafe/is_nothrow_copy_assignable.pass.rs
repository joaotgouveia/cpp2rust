extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TrivialNotNoexcept {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotEmpty {}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union Union {
    __empty: u8,
}
impl Default for Union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bit_zero {
    pub anon_0: (),
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A {}
impl A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonPODClass {}
impl NonPODClass {}
pub type Enum = ();
pub const Enum_EV: Enum = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Base {
    pub e: Enum,
    pub i: (),
    pub f: (),
    pub p: *mut NonPODClass,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Derived {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitConversionRvalueRef {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitConversionRef {}
pub unsafe fn test_has_nothrow_assign_1() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_2() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_3() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_4() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_5() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_6() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_7() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_8() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_nothrow_assign_9() {
    const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_not_nothrow_assign_10() {
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_not_nothrow_assign_11() {
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_not_nothrow_assign_12() {
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable_v<T>");;
}
pub unsafe fn test_has_not_nothrow_assign_13() {
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_assignable_v<T>");;
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
    (unsafe { test_has_nothrow_assign_1() });
    (unsafe { test_has_nothrow_assign_2() });
    (unsafe { test_has_nothrow_assign_3() });
    (unsafe { test_has_nothrow_assign_4() });
    (unsafe { test_has_nothrow_assign_5() });
    (unsafe { test_has_nothrow_assign_6() });
    (unsafe { test_has_nothrow_assign_7() });
    (unsafe { test_has_nothrow_assign_8() });
    (unsafe { test_has_nothrow_assign_9() });
    (unsafe { test_has_not_nothrow_assign_10() });
    (unsafe { test_has_not_nothrow_assign_11() });
    (unsafe { test_has_not_nothrow_assign_12() });
    (unsafe { test_has_not_nothrow_assign_13() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
