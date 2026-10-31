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
    pub anon_0: i32,
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A {}
impl A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonPODClass {}
impl NonPODClass {}
pub type Enum = u32;
pub const Enum_EV: Enum = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Base {
    pub e: Enum,
    pub i: i32,
    pub f: f32,
    pub p: *mut NonPODClass,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Derived {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_int__int_ref_ {
    pub obj: i32,
}
impl Default for ConvertsToRef_int__int_ref_ {
    fn default() -> Self {
        unsafe { ConvertsToRef_int__int_ref_ { obj: 0_i32 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_int__int_refref_ {
    pub obj: i32,
}
impl Default for ConvertsToRef_int__int_refref_ {
    fn default() -> Self {
        unsafe { ConvertsToRef_int__int_refref_ { obj: 0_i32 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_int__const_int_ref_ {
    pub obj: i32,
}
impl Default for ConvertsToRef_int__const_int_ref_ {
    fn default() -> Self {
        unsafe { ConvertsToRef_int__const_int_ref_ { obj: 0_i32 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_long__long_ref_ {
    pub obj: i64,
}
impl Default for ConvertsToRef_long__long_ref_ {
    fn default() -> Self {
        unsafe { ConvertsToRef_long__long_ref_ { obj: 0_i64 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRefPrivate_long__long_ref_ {
    obj: i64,
}
impl Default for ConvertsToRefPrivate_long__long_ref_ {
    fn default() -> Self {
        unsafe { ConvertsToRefPrivate_long__long_ref_ { obj: 0_i64 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitConversionRvalueRef {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitConversionRef {}
pub unsafe fn test_reference_converts_from_temporary_1() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_2() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_3() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_4() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_5() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_6() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_7() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_8() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_9() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_10() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_11() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_12() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_13() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_14() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_15() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_16() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_17() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_18() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_19() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_20() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_21() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_22() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_23() {
    assert!(((unimplemented!() as i32) == (false as i32)));
    assert!(((unimplemented!() as i32) == (false as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_24() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_reference_converts_from_temporary_25() {
    assert!(((unimplemented!() as i32) == (true as i32)));
    assert!(((unimplemented!() as i32) == (true as i32)));
}
pub unsafe fn test_26() -> bool {
    (unsafe { test_reference_converts_from_temporary_1() });
    (unsafe { test_reference_converts_from_temporary_1() });
    (unsafe { test_reference_converts_from_temporary_2() });
    (unsafe { test_reference_converts_from_temporary_3() });
    (unsafe { test_reference_converts_from_temporary_4() });
    (unsafe { test_reference_converts_from_temporary_5() });
    (unsafe { test_reference_converts_from_temporary_6() });
    (unsafe { test_reference_converts_from_temporary_7() });
    (unsafe { test_reference_converts_from_temporary_8() });
    (unsafe { test_reference_converts_from_temporary_9() });
    assert!((unimplemented!()));
    (unsafe { test_reference_converts_from_temporary_10() });
    assert!((unimplemented!()));
    (unsafe { test_reference_converts_from_temporary_11() });
    assert!((unimplemented!()));
    (unsafe { test_reference_converts_from_temporary_12() });
    assert!((unimplemented!()));
    (unsafe { test_reference_converts_from_temporary_13() });
    (unsafe { test_reference_converts_from_temporary_14() });
    (unsafe { test_reference_converts_from_temporary_15() });
    (unsafe { test_reference_converts_from_temporary_16() });
    (unsafe { test_reference_converts_from_temporary_17() });
    (unsafe { test_reference_converts_from_temporary_18() });
    (unsafe { test_reference_converts_from_temporary_19() });
    (unsafe { test_reference_converts_from_temporary_20() });
    (unsafe { test_reference_converts_from_temporary_5() });
    (unsafe { test_reference_converts_from_temporary_21() });
    (unsafe { test_reference_converts_from_temporary_22() });
    (unsafe { test_reference_converts_from_temporary_23() });
    (unsafe { test_reference_converts_from_temporary_24() });
    (unsafe { test_reference_converts_from_temporary_25() });
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
    (unsafe { test_26() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
