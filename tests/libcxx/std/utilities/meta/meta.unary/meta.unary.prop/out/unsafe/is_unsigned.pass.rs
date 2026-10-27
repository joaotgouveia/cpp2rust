extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_is_unsigned_0() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_unsigned_1() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_unsigned_2() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_unsigned_3() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_unsigned_4() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_unsigned_5() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_6() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_7() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_8() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_9() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_10() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_11() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_12() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_13() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_14() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_15() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_16() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_17() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_18() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_19() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_20() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_21() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_22() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_23() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_24() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_25() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_26() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_27() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_28() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_29() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_30() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_31() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_32() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub unsafe fn test_is_not_unsigned_33() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Class {}
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
    pub anon_34: (),
}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one: Enum = 1;
pub type EnumSigned = ();
pub const EnumSigned_two: EnumSigned = 0;
pub type EnumUnsigned = ();
pub const EnumUnsigned_three: EnumUnsigned = 0;
pub type EnumClass = ();
pub const EnumClass_zero: EnumClass = 0;
pub const EnumClass_one: EnumClass = 1;
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
    (unsafe { test_is_not_unsigned_6() });
    (unsafe { test_is_not_unsigned_7() });
    (unsafe { test_is_not_unsigned_8() });
    (unsafe { test_is_not_unsigned_9() });
    (unsafe { test_is_not_unsigned_10() });
    (unsafe { test_is_not_unsigned_11() });
    (unsafe { test_is_not_unsigned_12() });
    (unsafe { test_is_not_unsigned_13() });
    (unsafe { test_is_not_unsigned_14() });
    (unsafe { test_is_not_unsigned_15() });
    (unsafe { test_is_not_unsigned_16() });
    (unsafe { test_is_not_unsigned_17() });
    (unsafe { test_is_not_unsigned_18() });
    (unsafe { test_is_not_unsigned_19() });
    (unsafe { test_is_not_unsigned_20() });
    (unsafe { test_is_not_unsigned_21() });
    (unsafe { test_is_not_unsigned_22() });
    (unsafe { test_is_not_unsigned_23() });
    (unsafe { test_is_not_unsigned_24() });
    (unsafe { test_is_not_unsigned_25() });
    (unsafe { test_is_not_unsigned_26() });
    (unsafe { test_is_not_unsigned_27() });
    (unsafe { test_is_not_unsigned_28() });
    (unsafe { test_is_not_unsigned_29() });
    (unsafe { test_is_not_unsigned_30() });
    (unsafe { test_is_not_unsigned_31() });
    (unsafe { test_is_not_unsigned_32() });
    (unsafe { test_is_unsigned_0() });
    (unsafe { test_is_unsigned_1() });
    (unsafe { test_is_unsigned_2() });
    (unsafe { test_is_unsigned_3() });
    (unsafe { test_is_unsigned_4() });
    (unsafe { test_is_unsigned_2() });
    (unsafe { test_is_unsigned_5() });
    (unsafe { test_is_not_unsigned_33() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
