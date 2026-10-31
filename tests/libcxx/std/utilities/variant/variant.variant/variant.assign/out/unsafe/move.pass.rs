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
#[derive(FnPtrArg, Default)]
pub struct NoCopy {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CopyOnly {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveOnly {}
impl MoveOnly {
    pub unsafe fn move_from(_a0: *mut MoveOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnly) -> *mut MoveOnly {
        return &mut (*(self as *mut MoveOnly));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveOnlyNT {}
impl MoveOnlyNT {
    pub unsafe fn move_from(_a0: *mut MoveOnlyNT) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnlyNT) -> *mut MoveOnlyNT {
        return &mut (*(self as *mut MoveOnlyNT));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveOnlyOddNothrow {}
impl MoveOnlyOddNothrow {
    pub unsafe fn move_from(_a0: *mut MoveOnlyOddNothrow) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnlyOddNothrow) -> *mut MoveOnlyOddNothrow {
        return &mut (*(self as *mut MoveOnlyOddNothrow));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct MoveAssignOnly {}
impl MoveAssignOnly {
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssignOnly) -> *mut MoveAssignOnly {
        return &mut (*(self as *mut MoveAssignOnly));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveAssign {
    pub value: i32,
    pub move_construct: *mut i32,
    pub move_assign: *mut i32,
}
impl MoveAssign {
    pub unsafe fn new(mut v: i32, mut move_ctor: *mut i32, mut move_assi: *mut i32) -> Self {
        let mut this = Self {
            value: v,
            move_construct: move_ctor,
            move_assign: move_assi,
        };
        this
    }
    pub unsafe fn move_from(o: *mut MoveAssign) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_construct: (*o).move_construct,
            move_assign: (*o).move_assign,
        };
        (*this.move_construct).prefix_inc();
        (*o).value = -1_i32;
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut MoveAssign) -> *mut MoveAssign {
        self.value = (*o).value;
        self.move_construct = (*o).move_construct;
        self.move_assign = (*o).move_assign;
        (*self.move_assign).prefix_inc();
        (*o).value = -1_i32;
        return &mut (*(self as *mut MoveAssign));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NTMoveAssign {
    pub value: i32,
}
impl NTMoveAssign {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_assign(&mut self, that: *mut NTMoveAssign) -> *mut NTMoveAssign {
        self.value = (*that).value;
        (*that).value = -1_i32;
        return &mut (*(self as *mut NTMoveAssign));
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_move_assignable<NTMoveAssign>::value"
);
const _: () = assert!(true, "std::is_move_assignable<NTMoveAssign>::value");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct TMoveAssign {
    pub value: i32,
}
impl TMoveAssign {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_from(_a0: *mut TMoveAssign) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TMoveAssign) -> *mut TMoveAssign {
        self.value = (*_a0).value;
        return &mut (*(self as *mut TMoveAssign));
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_assignable<TMoveAssign>::value"
);
#[repr(C)]
#[derive(
    Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default,
)]
#[cpp_from(i32, new, value)]
pub struct TMoveAssignNTCopyAssign {
    pub value: i32,
}
impl TMoveAssignNTCopyAssign {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        that: *const TMoveAssignNTCopyAssign,
    ) -> *mut TMoveAssignNTCopyAssign {
        self.value = (*that).value;
        return &mut (*(self as *mut TMoveAssignNTCopyAssign));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut TMoveAssignNTCopyAssign,
    ) -> *mut TMoveAssignNTCopyAssign {
        self.value = (*_a0).value;
        return &mut (*(self as *mut TMoveAssignNTCopyAssign));
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_assignable_v<TMoveAssignNTCopyAssign>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct TrivialCopyNontrivialMove {}
impl TrivialCopyNontrivialMove {
    pub unsafe fn move_from(_a0: *mut TrivialCopyNontrivialMove) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut TrivialCopyNontrivialMove,
    ) -> *mut TrivialCopyNontrivialMove {
        return &mut (*(self as *mut TrivialCopyNontrivialMove));
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable_v<TrivialCopyNontrivialMove>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_move_assignable_v<TrivialCopyNontrivialMove>"
);
pub unsafe fn test_move_assignment_noexcept_5() {
    {
        const _: () = assert!(true, "std::is_nothrow_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_assignable<V>::value");;
    }
}
pub unsafe fn test_move_assignment_sfinae_6() {
    {
        const _: () = assert!(true, "std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_move_assignable<V>::value");;
        const _: () = assert!(true, "std::is_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_move_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    }
}
pub unsafe fn test_move_assignment_empty_empty_7() {}
pub unsafe fn test_move_assignment_non_empty_empty_8() {}
pub unsafe fn test_move_assignment_empty_non_empty_9() {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result_int_ {
    pub index: usize,
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result_long_ {
    pub index: usize,
    pub value: i64,
}
pub unsafe fn test_move_assignment_same_index_10() {
    {
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (0_usize)));
        assert!(((*unimplemented!()) == (42)));
    }
    {
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*unimplemented!()) == (42_())));
    }
    {
        let mut move_construct: i32 = 0;
        let mut move_assign: i32 = 0;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!((((*unimplemented!()).value) == (42)));
        assert!(((move_construct) == (0)));
        assert!(((move_assign) == (1)));
    }
    {
        let mut test: anon_11 = <anon_11>::default();
        let result: Result_int_ = (unsafe { anon_11::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_12 = <anon_12>::default();
        let result: Result_long_ = (unsafe { anon_12::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_13 = <anon_13>::default();
        let result: Result_int_ = (unsafe { anon_13::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_11 {}
impl anon_11 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ((*unimplemented!()) as i32),
        };
    }
}
impl Callable0<Result_int_> for anon_11 {
    fn call(&self) -> Result_int_ {
        unsafe { anon_11::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_12 {}
impl anon_12 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*unimplemented!()) as i64),
        };
    }
}
impl Callable0<Result_long_> for anon_12 {
    fn call(&self) -> Result_long_ {
        unsafe { anon_12::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_13 {}
impl anon_13 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*unimplemented!()).value,
        };
    }
}
impl Callable0<Result_int_> for anon_13 {
    fn call(&self) -> Result_int_ {
        unsafe { anon_13::operator_call(self) }
    }
}
pub unsafe fn test_move_assignment_different_index_14() {
    {
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*unimplemented!()) == (42_())));
    }
    {
        let mut move_construct: i32 = 0;
        let mut move_assign: i32 = 0;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        assert!(((vref) == (&mut v1 as *mut ())));
        assert!(((unimplemented!()) == (1_usize)));
        assert!((((*unimplemented!()).value) == (42)));
        assert!(((move_construct) == (1)));
        assert!(((move_assign) == (0)));
    }
    {
        let mut test: anon_15 = <anon_15>::default();
        let result: Result_long_ = (unsafe { anon_15::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_16 = <anon_16>::default();
        let result: Result_long_ = (unsafe { anon_16::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_15 {}
impl anon_15 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*unimplemented!()) as i64),
        };
    }
}
impl Callable0<Result_long_> for anon_15 {
    fn call(&self) -> Result_long_ {
        unsafe { anon_15::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_16 {}
impl anon_16 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*unimplemented!()).value as i64),
        };
    }
}
impl Callable0<Result_long_> for anon_16 {
    fn call(&self) -> Result_long_ {
        unsafe { anon_16::operator_call(self) }
    }
}
pub unsafe fn test_assignment_throw_17() {}
pub unsafe fn test_constexpr_assign_imp_18(v: *mut (), new_value: *mut i64) {
    let mut v2: () = unimplemented!();
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_19(v: *mut (), new_value: *mut ()) {
    let mut v2: () = unimplemented!();
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_20(v: *mut (), new_value: *mut i32) {
    let mut v2: () = unimplemented!();
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (2_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_21(v: *mut (), new_value: *mut i64) {
    let mut v2: () = unimplemented!();
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (0_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_22(v: *mut (), new_value: *mut ()) {
    let mut v2: () = unimplemented!();
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (1_u64 as usize)));
    assert!(((*unimplemented!()) == (*unimplemented!())));
}
pub unsafe fn test_constexpr_assign_imp_23(v: *mut (), new_value: *mut NonTrivialMoveAssign) {
    let mut v2: () = unimplemented!();
    let cp: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (2_u64 as usize)));
    assert!(
        (unsafe {
            let _x: *const NonTrivialMoveAssign = unimplemented!();
            operator_eq_24(_x, unimplemented!())
        })
    );
}
pub unsafe fn test_constexpr_move_assignment_trivial_25() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_18(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_18(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_19(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i32 = 101;
        test_constexpr_assign_imp_20(&mut _v, &mut _new_value)
    });
}
pub unsafe fn operator_eq_24(
    x: *const NonTrivialMoveAssign,
    y: *const NonTrivialMoveAssign,
) -> bool {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, value)]
pub struct NonTrivialMoveAssign {
    pub i: i32,
}
impl NonTrivialMoveAssign {
    pub unsafe fn new(mut ii: i32) -> Self {
        let mut this = Self { i: ii };
        this
    }
    pub unsafe fn move_from(other: *mut NonTrivialMoveAssign) -> Self {
        let mut this = Self { i: (*other).i };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut NonTrivialMoveAssign,
    ) -> *mut NonTrivialMoveAssign {
        self.i = (*o).i;
        return &mut (*(self as *mut NonTrivialMoveAssign));
    }
}
impl std::cmp::PartialEq for NonTrivialMoveAssign {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_24(
                self as *const NonTrivialMoveAssign,
                other as *const NonTrivialMoveAssign,
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialMoveAssign {}
impl Default for NonTrivialMoveAssign {
    fn default() -> Self {
        unsafe { NonTrivialMoveAssign { i: 0 } }
    }
}
pub unsafe fn test_constexpr_move_assignment_non_trivial_26() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable<V>::value");;
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_21(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: i64 = 101_i64;
        test_constexpr_assign_imp_21(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_22(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: NonTrivialMoveAssign = NonTrivialMoveAssign::new({ 5 });
        test_constexpr_assign_imp_23(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = unimplemented!();
        let mut _new_value: NonTrivialMoveAssign = NonTrivialMoveAssign::new({ 5 });
        test_constexpr_assign_imp_23(&mut _v, &mut _new_value)
    });
}
pub unsafe fn non_constexpr_test_27() {
    (unsafe { test_move_assignment_empty_empty_7() });
    (unsafe { test_move_assignment_non_empty_empty_8() });
    (unsafe { test_move_assignment_empty_non_empty_9() });
    (unsafe { test_assignment_throw_17() });
}
pub unsafe fn cxx17_constexpr_test_28() -> bool {
    (unsafe { test_move_assignment_sfinae_6() });
    (unsafe { test_move_assignment_noexcept_5() });
    (unsafe { test_constexpr_move_assignment_trivial_25() });
    return true;
}
pub unsafe fn cxx20_constexpr_test_29() -> bool {
    (unsafe { test_move_assignment_same_index_10() });
    (unsafe { test_move_assignment_different_index_14() });
    (unsafe { test_constexpr_move_assignment_non_trivial_26() });
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
    (unsafe { non_constexpr_test_27() });
    (unsafe { cxx17_constexpr_test_28() });
    (unsafe { cxx20_constexpr_test_29() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
