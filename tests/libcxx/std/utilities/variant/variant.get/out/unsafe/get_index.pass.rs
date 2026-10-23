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
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_long_ref__const_long_ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_refref__int_refref_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_long_refref__const_long_refref_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_refref__const_int_refref_ {}
pub unsafe fn operator_eq_6(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_7(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const (),
}
impl TypeID {
    pub unsafe fn name(&self) -> () {
        return unimplemented!();
    }
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub unsafe fn operator_bitor_8(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as ()) | (RHS as ())) as CallType);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardingCallObject {}
impl ForwardingCallObject {
    pub unsafe fn to_(&self) -> () {
        return 0_();
    }
}
pub static mut last_call_type_9: std::cell::LazyCell<CallType> =
    std::cell::LazyCell::new(|| unsafe { CallType_CT_None });
pub static mut last_call_args_10: std::cell::LazyCell<*const TypeID> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnFirst {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnArity {}
pub unsafe fn test_const_lvalue_get_11() {
    {
        let v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::get<0>(v) == 42");;
    }
    {
        let v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub unsafe fn test_lvalue_get_12() {
    {
        let mut v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut v: () = 42_();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub unsafe fn test_rvalue_get_13() {
    {
        let mut v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let mut v: () = 42_();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub unsafe fn test_const_rvalue_get_14() {
    {
        let v: () = 42_();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let v: () = 42_();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub unsafe fn test_throws_for_all_value_categories_15() {}
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
    (unsafe { test_const_lvalue_get_11() });
    (unsafe { test_lvalue_get_12() });
    (unsafe { test_rvalue_get_13() });
    (unsafe { test_const_rvalue_get_14() });
    (unsafe { test_throws_for_all_value_categories_15() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_9);
    std::cell::LazyCell::force(&*&raw const last_call_args_10);
}
