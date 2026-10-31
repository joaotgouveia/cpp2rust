extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(LHS: *const TypeID, RHS: *const TypeID) -> bool {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_1(LHS: *const TypeID, RHS: *const TypeID) -> bool {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const libc::c_char,
}
impl TypeID {
    pub unsafe fn name(&self) -> Vec<libc::c_char> {
        return std::slice::from_raw_parts(self.m_id, libcc2rs::c_str_len(self.m_id) + 1).to_vec();
    }
    unsafe fn new(mut xid: *const libc::c_char) -> Self {
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
pub type CallType = u32;
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub unsafe fn operator_bitor_2(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as u32) | (RHS as u32)) as CallType);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct ForwardingCallObject {}
impl ForwardingCallObject {
    pub unsafe fn to_i32(&self) -> i32 {
        return 0;
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NonDefaultConstructible {}
impl NonDefaultConstructible {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NotNoexcept {}
impl NotNoexcept {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for NotNoexcept {
    fn default() -> Self {
        unsafe { NotNoexcept::new() }
    }
}
pub unsafe fn test_default_ctor_sfinae_5() {
    {
        const _: () = assert!(true, "std::is_default_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_default_constructible<V>::value");;
    }
}
pub unsafe fn test_default_ctor_noexcept_6() {
    {
        const _: () = assert!(true, "std::is_nothrow_default_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_default_constructible<V>::value");;
    }
}
pub unsafe fn test_default_ctor_throws_7() {}
pub unsafe fn test_default_ctor_basic_8() {
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (0)));
    }
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (0)));
    }
    {
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (0)));
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 0");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 0");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 0");;
    }
}
pub unsafe fn issue_86686_9() {
    const _: () = assert!(true, "std::variant<std::string>{{}}.index() == 0");;
}
pub unsafe fn test_10() -> bool {
    (unsafe { test_default_ctor_basic_8() });
    (unsafe { test_default_ctor_sfinae_5() });
    (unsafe { test_default_ctor_noexcept_6() });
    (unsafe { issue_86686_9() });
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
    (unsafe { test_10() });
    (unsafe { test_default_ctor_throws_7() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
