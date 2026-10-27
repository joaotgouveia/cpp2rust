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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Dummy {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThrowsT {}
impl ThrowsT {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoThrowT {}
impl NoThrowT {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
impl AnyConstructible {
    pub unsafe fn new(_a0: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AnyConstructible {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoConstructible {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RValueConvertibleFrom_int_ {}
impl RValueConvertibleFrom_int_ {
    pub unsafe fn new(_a0: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test_T_ctor_noexcept_5() {
    {
        const _: () = assert!(true, "std::is_nothrow_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_constructible<V, int>::value");;
    }
}
pub unsafe fn test_T_ctor_sfinae_6() {
    {
        const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<V, const char*>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::unique_ptr<char>>::value"
        );;
        const _: () = assert!(true, "!std::is_constructible<V, X>::value");;
        const _: () = assert!(true, "std::is_constructible<V, std::false_type>::value");;
    }
    {
        const _: () = assert!(true, "std::is_constructible<V, Y>::value");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<NoConstructible>>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<1>>::value"
        );;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X_7 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X_8 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Y_9 {}
pub unsafe fn test_T_ctor_basic_10() {
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v1: () = unimplemented!();
        unimplemented!();
        let mut x: () = 42_();
        let mut v2: () = unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BoomOnAnything {}
pub unsafe fn test_no_narrowing_check_for_class_types_11() {
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Baz {}
pub unsafe fn test_construction_with_repeated_types_12() {
    const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    const _: () = assert!(true, "!std::is_constructible<V, Baz>::value");;
    const _: () = assert!(true, "std::is_constructible<V, Bar>::value");;
}
pub unsafe fn test_vector_bool_13() {
    let mut vec_: () = unimplemented!();
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleFromAny {}
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
    (unsafe { test_T_ctor_basic_10() });
    (unsafe { test_T_ctor_noexcept_5() });
    (unsafe { test_T_ctor_sfinae_6() });
    (unsafe { test_no_narrowing_check_for_class_types_11() });
    (unsafe { test_construction_with_repeated_types_12() });
    (unsafe { test_vector_bool_13() });
    {
        let mut vec_: Matcher_14 = Matcher_14::new_1();
        let mut m: Matcher_14 = vec_;
    }
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Matcher_14 {}
impl Matcher_14 {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for Matcher_14 {
    fn default() -> Self {
        unsafe { Matcher_14::new_1() }
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
