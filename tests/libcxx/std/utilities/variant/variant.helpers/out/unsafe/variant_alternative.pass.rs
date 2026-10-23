extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_1(LHS: *const TypeID, RHS: *const TypeID) -> () {
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
        unsafe { operator_eq_0(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub unsafe fn operator_bitor_2(mut LHS: CallType, mut RHS: CallType) -> CallType {
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
pub static mut last_call_type_3: std::cell::LazyCell<CallType> =
    std::cell::LazyCell::new(|| unsafe { CallType_CT_None });
pub static mut last_call_args_4: std::cell::LazyCell<*const TypeID> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnFirst {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnArity {}
pub unsafe fn test_5() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
}
pub unsafe fn test_6() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
}
pub unsafe fn test_7() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
}
pub unsafe fn test_8() {
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, V>::type, E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, const V>::type,\n                     const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<typename std::variant_alternative<I, volatile V>::type,\n                     volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n          typename std::variant_alternative<I, const volatile V>::type,\n          const volatile E>"
    );;
    const _: () = assert!(true, "std::is_same_v<std::variant_alternative_t<I, V>, E>");;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const V>, const E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, volatile V>, volatile E>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<std::variant_alternative_t<I, const volatile V>,\n                               const volatile E>"
    );;
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
    {
        (unsafe { test_5() });
        (unsafe { test_6() });
        (unsafe { test_7() });
        (unsafe { test_8() });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
