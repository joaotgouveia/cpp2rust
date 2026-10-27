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
pub struct IntType {
    pub val: (),
}
impl IntType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&self) -> () {
        return self.val;
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { IntType::operator_eq(self, other as *const IntType) }
    }
}
impl std::cmp::Eq for IntType {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntTypeNC {
    pub val: (),
}
impl IntTypeNC {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&mut self) -> () {
        return self.val;
    }
}
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long___std_extents_unsigned_long__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__18446744073709551615___std_extents_unsigned_long__18446744073709551615__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_extents_unsigned_long__18446744073709551615__18446744073709551615__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615___std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__5___std_extents_unsigned_long__5__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__5__18446744073709551615___std_extents_unsigned_long__5__18446744073709551615__
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__5__18446744073709551615__7___std_extents_unsigned_long__5__18446744073709551615__7__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoDefaultCtorIndex {
    pub value: (),
}
impl NoDefaultCtorIndex {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self { value: val };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.value;
    }
}
pub unsafe fn test_7(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_8(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_9(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_10(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_11(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_12(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_13(mut e: (), mut expected: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_14() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_15));
    (unsafe { test_7(unimplemented!(), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
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
    (unsafe { test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
