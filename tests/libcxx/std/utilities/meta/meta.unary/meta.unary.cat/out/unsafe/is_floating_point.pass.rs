extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_is_floating_point_0() {
    const _: () = assert!(true, "std::is_floating_point<T>::value");;
    const _: () = assert!(true, "std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "std::is_floating_point_v<T>");;
    const _: () = assert!(true, "std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_floating_point_1() {
    const _: () = assert!(true, "std::is_floating_point<T>::value");;
    const _: () = assert!(true, "std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "std::is_floating_point_v<T>");;
    const _: () = assert!(true, "std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_floating_point_2() {
    const _: () = assert!(true, "std::is_floating_point<T>::value");;
    const _: () = assert!(true, "std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "std::is_floating_point_v<T>");;
    const _: () = assert!(true, "std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_3() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_4() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_5() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_6() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_7() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_8() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_9() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_10() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_11() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_12() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_13() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_14() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_15() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_16() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_17() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_18() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_19() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_20() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_21() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_22() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_23() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
pub unsafe fn test_is_not_floating_point_24() {
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_floating_point_v<T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<volatile T>");;
    const _: () = assert!(true, "!std::is_floating_point_v<const volatile T>");;
}
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
    pub anon_25: (),
}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one: Enum = 1;
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
    (unsafe { test_is_floating_point_0() });
    (unsafe { test_is_floating_point_1() });
    (unsafe { test_is_floating_point_2() });
    (unsafe { test_is_not_floating_point_3() });
    (unsafe { test_is_not_floating_point_4() });
    (unsafe { test_is_not_floating_point_5() });
    (unsafe { test_is_not_floating_point_6() });
    (unsafe { test_is_not_floating_point_7() });
    (unsafe { test_is_not_floating_point_8() });
    (unsafe { test_is_not_floating_point_9() });
    (unsafe { test_is_not_floating_point_10() });
    (unsafe { test_is_not_floating_point_11() });
    (unsafe { test_is_not_floating_point_12() });
    (unsafe { test_is_not_floating_point_13() });
    (unsafe { test_is_not_floating_point_14() });
    (unsafe { test_is_not_floating_point_15() });
    (unsafe { test_is_not_floating_point_16() });
    (unsafe { test_is_not_floating_point_17() });
    (unsafe { test_is_not_floating_point_18() });
    (unsafe { test_is_not_floating_point_19() });
    (unsafe { test_is_not_floating_point_20() });
    (unsafe { test_is_not_floating_point_21() });
    (unsafe { test_is_not_floating_point_22() });
    (unsafe { test_is_not_floating_point_23() });
    (unsafe { test_is_not_floating_point_24() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
