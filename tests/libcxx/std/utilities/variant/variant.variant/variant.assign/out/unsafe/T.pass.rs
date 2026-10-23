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
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { self.m_id },
            None,
        );
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
pub struct MetaHelpers_Dummy {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MetaHelpers_ThrowsCtorT {}
impl MetaHelpers_ThrowsCtorT {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_2(&mut self, mut _a0: ()) -> *mut MetaHelpers_ThrowsCtorT {
        return &mut (*(self as *mut MetaHelpers_ThrowsCtorT));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MetaHelpers_ThrowsAssignT {}
impl MetaHelpers_ThrowsAssignT {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_2(&mut self, mut _a0: ()) -> *mut MetaHelpers_ThrowsAssignT {
        return &mut (*(self as *mut MetaHelpers_ThrowsAssignT));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MetaHelpers_NoThrowT {}
impl MetaHelpers_NoThrowT {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_2(&mut self, mut _a0: ()) -> *mut MetaHelpers_NoThrowT {
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
        let mut v: () = 43_();
        v = 42_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = 43_();
        v = 42_();
        unimplemented!();
        unimplemented!();
        v = 43_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = unimplemented!();
        v = 42_();
        unimplemented!();
        unimplemented!();
        v = 43_();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v: () = true;
        v = c"bar";
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_T_assignment_basic_no_constexpr_11() {
    let mut v: () = unimplemented!();
    v = Default::default();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct TraceStat {
    pub construct: (),
    pub copy_construct: (),
    pub copy_assign: (),
    pub move_construct: (),
    pub move_assign: (),
    pub T_copy_assign: (),
    pub T_move_assign: (),
    pub destroy: (),
}
impl Default for TraceStat {
    fn default() -> Self {
        unsafe {
            TraceStat {
                construct: 0_(),
                copy_construct: 0_(),
                copy_assign: 0_(),
                move_construct: 0_(),
                move_assign: 0_(),
                T_copy_assign: 0_(),
                T_move_assign: 0_(),
                destroy: 0_(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trace_false__false__T {}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, DestructorUnsafe, Default)]
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
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
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
#[derive(FnPtrArg, MoveAssignUnsafe, DestructorUnsafe, Default)]
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
#[derive(FnPtrArg, MoveAssignUnsafe, DestructorUnsafe, Default)]
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
        let mut v: () = 1_();
        v = (&mut stat as *mut TraceStat);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = 1_();
        v = (&mut stat as *mut TraceStat);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = 1_();
        v = (&mut stat as *mut TraceStat);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = 1_();
        v = (&mut stat as *mut TraceStat);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_T_assignment_performs_assignment_13() {
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = (&mut stat as *mut TraceStat);
        v = <Trace_false__false__T>::default();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut stat: TraceStat = <TraceStat>::default();
        let mut v: () = (&mut stat as *mut TraceStat);
        let mut t: Trace_false__false__T = <Trace_false__false__T>::default();
        v = t;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_T_assignment_performs_construction_throw_14() {}
pub unsafe fn test_T_assignment_performs_assignment_throw_15() {}
pub unsafe fn test_T_assignment_vector_bool_16() {
    let mut vec_: () = std_vector_bool__std_allocator_bool__::new_1({ unimplemented!() });
    let mut v: () = unimplemented!();
    v = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn non_constexpr_test_17() {
    (unsafe { test_T_assignment_basic_no_constexpr_11() });
    (unsafe { test_T_assignment_performs_construction_throw_14() });
    (unsafe { test_T_assignment_performs_assignment_throw_15() });
}
pub unsafe fn test_18() -> () {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_18() });
    (unsafe { non_constexpr_test_17() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
