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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_int__int_ref_ {
    pub obj: (),
}
impl Default for ConvertsToRef_int__int_ref_ {
    fn default() -> Self {
        unsafe {
            ConvertsToRef_int__int_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_int__int_refref_ {
    pub obj: (),
}
impl Default for ConvertsToRef_int__int_refref_ {
    fn default() -> Self {
        unsafe {
            ConvertsToRef_int__int_refref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_int__const_int_ref_ {
    pub obj: (),
}
impl Default for ConvertsToRef_int__const_int_ref_ {
    fn default() -> Self {
        unsafe {
            ConvertsToRef_int__const_int_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRef_long__long_ref_ {
    pub obj: (),
}
impl Default for ConvertsToRef_long__long_ref_ {
    fn default() -> Self {
        unsafe {
            ConvertsToRef_long__long_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsToRefPrivate_long__long_ref_ {
    obj: (),
}
impl Default for ConvertsToRefPrivate_long__long_ref_ {
    fn default() -> Self {
        unsafe {
            ConvertsToRefPrivate_long__long_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitConversionRvalueRef {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitConversionRef {}
pub unsafe fn test_reference_converts_from_temporary_1() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_2() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_3() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_4() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_5() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_6() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_7() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_8() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_9() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_10() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_11() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_12() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_13() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_14() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_15() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_16() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_17() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_18() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_19() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_20() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_21() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_22() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_23() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_24() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_reference_converts_from_temporary_25() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_26() -> () {
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
    unimplemented!();
    (unsafe { test_reference_converts_from_temporary_10() });
    unimplemented!();
    (unsafe { test_reference_converts_from_temporary_11() });
    unimplemented!();
    (unsafe { test_reference_converts_from_temporary_12() });
    unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_26() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
