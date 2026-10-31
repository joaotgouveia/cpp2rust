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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MetaHelpers_Dummy {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct MetaHelpers_ThrowsCtorT {}
impl MetaHelpers_ThrowsCtorT {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_2(&mut self, mut _a0: i32) -> *mut MetaHelpers_ThrowsCtorT {
        return &mut (*(self as *mut MetaHelpers_ThrowsCtorT));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct MetaHelpers_ThrowsAssignT {}
impl MetaHelpers_ThrowsAssignT {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_2(&mut self, mut _a0: i32) -> *mut MetaHelpers_ThrowsAssignT {
        return &mut (*(self as *mut MetaHelpers_ThrowsAssignT));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct MetaHelpers_NoThrowT {}
impl MetaHelpers_NoThrowT {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_2(&mut self, mut _a0: i32) -> *mut MetaHelpers_NoThrowT {
        return &mut (*(self as *mut MetaHelpers_NoThrowT));
    }
}
pub unsafe fn test_T_assignment_noexcept_5() {
    {
        const _: () = assert!(true, "std::is_nothrow_assignable<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_assignable<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_assignable<V, int>::value");;
    }
}
pub unsafe fn test_T_assignment_sfinae_6() {
    {
        const _: () = assert!(true, "!std::is_assignable<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<V, const char*>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<V, std::unique_ptr<char>>::value");;
        const _: () = assert!(true, "!std::is_assignable<V, X>::value");;
        const _: () = assert!(true, "std::is_assignable<V, std::false_type>::value");;
    }
    {
        const _: () = assert!(true, "std::is_assignable<V, Y>::value");;
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
pub unsafe fn test_T_assignment_basic_10() {
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (42)));
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (42)));
        unimplemented!();
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*unimplemented!()) == (43_())));
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*unimplemented!()) == (42_())));
        unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (43_())));
    }
    {
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (0_usize)));
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"bar".as_ptr(),
                    libcc2rs::c_str_len(c"bar".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test_T_assignment_basic_no_constexpr_11() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (1_usize)));
    assert!((*unimplemented!()).is_none());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct TraceStat {
    pub construct: i32,
    pub copy_construct: i32,
    pub copy_assign: i32,
    pub move_construct: i32,
    pub move_assign: i32,
    pub T_copy_assign: i32,
    pub T_move_assign: i32,
    pub destroy: i32,
}
impl Default for TraceStat {
    fn default() -> Self {
        unsafe {
            TraceStat {
                construct: 0,
                copy_construct: 0,
                copy_assign: 0,
                move_construct: 0,
                move_assign: 0,
                T_copy_assign: 0,
                T_move_assign: 0,
                destroy: 0,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trace_false__false__T {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(*mut TraceStat , new_1, value)]
#[cpp_from(Trace_false__false__T, new_2, value)]
pub struct Trace_false__false_ {
    pub stat: *mut TraceStat,
}
impl Trace_false__false_ {
    pub unsafe fn new_1(mut s: *mut TraceStat) -> Self {
        let mut this = Self { stat: s };
        (*s).construct.prefix_inc();
        this
    }
    pub unsafe fn new_2(mut _a0: Trace_false__false__T) -> Self {
        let mut this = Self {
            stat: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut Trace_false__false_,
    ) -> *mut Trace_false__false_ {
        (*self.stat).move_assign.prefix_inc();
        return &mut (*(self as *mut Trace_false__false_));
    }
    pub unsafe fn operator_assign_7(
        &mut self,
        _a0: *const Trace_false__false__T,
    ) -> *mut Trace_false__false_ {
        (*self.stat).T_copy_assign.prefix_inc();
        return &mut (*(self as *mut Trace_false__false_));
    }
    pub unsafe fn operator_assign_8(
        &mut self,
        _a0: *mut Trace_false__false__T,
    ) -> *mut Trace_false__false_ {
        (*self.stat).T_move_assign.prefix_inc();
        return &mut (*(self as *mut Trace_false__false_));
    }
    pub unsafe fn destructor(&mut self) {
        (*self.stat).destroy.prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trace_false__true__T {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(*mut TraceStat , new_1, value)]
pub struct Trace_false__true_ {
    pub stat: *mut TraceStat,
}
impl Trace_false__true_ {
    pub unsafe fn new_1(mut s: *mut TraceStat) -> Self {
        let mut this = Self { stat: s };
        (*s).construct.prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Trace_false__true_) -> Self {
        let mut this = Self { stat: (*o).stat };
        (*this.stat).move_construct.prefix_inc();
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Trace_false__true_) -> *mut Trace_false__true_ {
        (*self.stat).move_assign.prefix_inc();
        return &mut (*(self as *mut Trace_false__true_));
    }
    pub unsafe fn destructor(&mut self) {
        (*self.stat).destroy.prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trace_true__false__T {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(*mut TraceStat , new_1, value)]
pub struct Trace_true__false_ {
    pub stat: *mut TraceStat,
}
impl Trace_true__false_ {
    pub unsafe fn new_1(mut s: *mut TraceStat) -> Self {
        let mut this = Self { stat: s };
        (*s).construct.prefix_inc();
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Trace_true__false_) -> *mut Trace_true__false_ {
        (*self.stat).move_assign.prefix_inc();
        return &mut (*(self as *mut Trace_true__false_));
    }
    pub unsafe fn destructor(&mut self) {
        (*self.stat).destroy.prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trace_true__true__T {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(*mut TraceStat , new_1, value)]
pub struct Trace_true__true_ {
    pub stat: *mut TraceStat,
}
impl Trace_true__true_ {
    pub unsafe fn new_1(mut s: *mut TraceStat) -> Self {
        let mut this = Self { stat: s };
        (*s).construct.prefix_inc();
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Trace_true__true_) -> *mut Trace_true__true_ {
        (*self.stat).move_assign.prefix_inc();
        return &mut (*(self as *mut Trace_true__true_));
    }
    pub unsafe fn destructor(&mut self) {
        (*self.stat).destroy.prefix_inc();
    }
}
pub unsafe fn test_T_assignment_performs_construction_12() {
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((stat.construct) == (1)));
        assert!(((stat.copy_construct) == (0)));
        assert!(((stat.move_construct) == (0)));
        assert!(((stat.copy_assign) == (0)));
        assert!(((stat.move_assign) == (0)));
        assert!(((stat.destroy) == (0)));
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((stat.construct) == (1)));
        assert!(((stat.copy_construct) == (0)));
        assert!(((stat.move_construct) == (1)));
        assert!(((stat.copy_assign) == (0)));
        assert!(((stat.move_assign) == (0)));
        assert!(((stat.destroy) == (1)));
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((stat.construct) == (1)));
        assert!(((stat.copy_construct) == (0)));
        assert!(((stat.move_construct) == (0)));
        assert!(((stat.copy_assign) == (0)));
        assert!(((stat.move_assign) == (0)));
        assert!(((stat.destroy) == (0)));
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((stat.construct) == (1)));
        assert!(((stat.copy_construct) == (0)));
        assert!(((stat.move_construct) == (0)));
        assert!(((stat.copy_assign) == (0)));
        assert!(((stat.move_assign) == (0)));
        assert!(((stat.destroy) == (0)));
    }
}
pub unsafe fn test_T_assignment_performs_assignment_13() {
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((stat.construct) == (1)));
        assert!(((stat.copy_construct) == (0)));
        assert!(((stat.move_construct) == (0)));
        assert!(((stat.copy_assign) == (0)));
        assert!(((stat.move_assign) == (0)));
        assert!(((stat.T_copy_assign) == (0)));
        assert!(((stat.T_move_assign) == (1)));
        assert!(((stat.destroy) == (0)));
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = unimplemented!();
        let mut t: Trace_false__false__T = <Trace_false__false__T>::default();
        unimplemented!();
        assert!(((stat.construct) == (1)));
        assert!(((stat.copy_construct) == (0)));
        assert!(((stat.move_construct) == (0)));
        assert!(((stat.copy_assign) == (0)));
        assert!(((stat.move_assign) == (0)));
        assert!(((stat.T_copy_assign) == (1)));
        assert!(((stat.T_move_assign) == (0)));
        assert!(((stat.destroy) == (0)));
    }
}
pub unsafe fn test_T_assignment_performs_construction_throw_14() {}
pub unsafe fn test_T_assignment_performs_assignment_throw_15() {}
pub unsafe fn test_T_assignment_vector_bool_16() {
    let mut vec_: Vec<bool> = Vec::from([true]);
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (0_usize)));
    assert!((((*unimplemented!()) as i32) == (true as i32)));
}
pub unsafe fn non_constexpr_test_17() {
    (unsafe { test_T_assignment_basic_no_constexpr_11() });
    (unsafe { test_T_assignment_performs_construction_throw_14() });
    (unsafe { test_T_assignment_performs_assignment_throw_15() });
}
pub unsafe fn test_18() -> bool {
    (unsafe { test_T_assignment_basic_10() });
    (unsafe { test_T_assignment_performs_construction_12() });
    (unsafe { test_T_assignment_performs_assignment_13() });
    (unsafe { test_T_assignment_noexcept_5() });
    (unsafe { test_T_assignment_sfinae_6() });
    (unsafe { test_T_assignment_vector_bool_16() });
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
    (unsafe { test_18() });
    (unsafe { non_constexpr_test_17() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
