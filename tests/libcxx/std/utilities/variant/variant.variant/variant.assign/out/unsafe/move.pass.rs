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
    pub value: (),
    pub move_construct: *mut (),
    pub move_assign: *mut (),
}
impl MoveAssign {
    pub unsafe fn new(mut v: (), mut move_ctor: *mut (), mut move_assi: *mut ()) -> Self {
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
        (*o).value = -1_();
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut MoveAssign) -> *mut MoveAssign {
        self.value = (*o).value;
        self.move_construct = (*o).move_construct;
        self.move_assign = (*o).move_assign;
        (*self.move_assign).prefix_inc();
        (*o).value = -1_();
        return &mut (*(self as *mut MoveAssign));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveAssignUnsafe, Default)]
pub struct NTMoveAssign {
    pub value: (),
}
impl NTMoveAssign {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn move_assign(&mut self, that: *mut NTMoveAssign) -> *mut NTMoveAssign {
        self.value = (*that).value;
        (*that).value = -1_();
        return &mut (*(self as *mut NTMoveAssign));
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_move_assignable<NTMoveAssign>::value"
);
const _: () = assert!(true, "std::is_move_assignable<NTMoveAssign>::value");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct TMoveAssign {
    pub value: (),
}
impl TMoveAssign {
    pub unsafe fn new(mut v: ()) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct TMoveAssignNTCopyAssign {
    pub value: (),
}
impl TMoveAssignNTCopyAssign {
    pub unsafe fn new(mut v: ()) -> Self {
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
    pub index: (),
    pub value: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result_long_ {
    pub index: (),
    pub value: (),
}
pub unsafe fn test_move_assignment_same_index_10() {
    {
        let mut v1: () = 43_();
        let mut v2: () = 42_();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut v1: () = 43_();
        let mut v2: () = 42_();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut move_construct: () = 0_();
        let mut move_assign: () = 0_();
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut test: anon_11 = <anon_11>::default();
        let result: Result_int_ = (unsafe { anon_11::operator_call(&test) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let mut test: anon_13 = <anon_13>::default();
        let result: Result_long_ = (unsafe { anon_13::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_15 = <anon_15>::default();
        let result: Result_int_ = (unsafe { anon_15::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_11 {}
impl anon_11 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = 43_();
        let mut v2: () = 42_();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_12(&mut v) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_13 {}
impl anon_13 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = 43_();
        let mut v2: () = 42_();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_14(&mut v) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_15 {}
impl anon_15 {
    pub unsafe fn operator_call(&self) -> Result_int_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (*(unsafe { get_16(&mut v) })).value,
        };
    }
}
pub unsafe fn test_move_assignment_different_index_17() {
    {
        let mut v1: () = 43_();
        let mut v2: () = 42_();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut move_construct: () = 0_();
        let mut move_assign: () = 0_();
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        let vref: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut test: anon_18 = <anon_18>::default();
        let result: Result_long_ = (unsafe { anon_18::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let mut test: anon_19 = <anon_19>::default();
        let result: Result_long_ = (unsafe { anon_19::operator_call(&test) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_18 {}
impl anon_18 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = 43_();
        let mut v2: () = 42_();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ((*(unsafe { get_14(&mut v) })) as ()),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_19 {}
impl anon_19 {
    pub unsafe fn operator_call(&self) -> Result_long_ {
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: (*(unsafe { get_16(&mut v) })).value,
        };
    }
}
pub unsafe fn test_assignment_throw_20() {}
pub unsafe fn test_constexpr_assign_imp_21(v: *mut (), new_value: *mut ()) {
    let mut v2: std_variant_long__void_ptr__int_ = (*new_value);
    let cp: std_variant_long__void_ptr__int_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_22(v: *mut (), new_value: *mut ()) {
    let mut v2: std_variant_long__void_ptr__int_ = (*new_value);
    let cp: std_variant_long__void_ptr__int_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_23(v: *mut (), new_value: *mut ()) {
    let mut v2: std_variant_long__void_ptr__int_ = (*new_value);
    let cp: std_variant_long__void_ptr__int_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_24(v: *mut (), new_value: *mut ()) {
    let mut v2: std_variant_long__void_ptr__NonTrivialMoveAssign_ = (*new_value);
    let cp: std_variant_long__void_ptr__NonTrivialMoveAssign_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_25(v: *mut (), new_value: *mut ()) {
    let mut v2: std_variant_long__void_ptr__NonTrivialMoveAssign_ = (*new_value);
    let cp: std_variant_long__void_ptr__NonTrivialMoveAssign_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_assign_imp_26(v: *mut (), new_value: *mut NonTrivialMoveAssign) {
    let mut v2: std_variant_long__void_ptr__NonTrivialMoveAssign_ = (*new_value);
    let cp: std_variant_long__void_ptr__NonTrivialMoveAssign_ = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_constexpr_move_assignment_trivial_27() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_21(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = Default::default();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_21(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_22(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_23(&mut _v, &mut _new_value)
    });
}
pub unsafe fn operator_eq_28(x: *const NonTrivialMoveAssign, y: *const NonTrivialMoveAssign) -> () {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NonTrivialMoveAssign {
    pub i: (),
}
impl NonTrivialMoveAssign {
    pub unsafe fn new(mut ii: ()) -> Self {
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
            operator_eq_28(
                self as *const NonTrivialMoveAssign,
                other as *const NonTrivialMoveAssign,
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialMoveAssign {}
impl Default for NonTrivialMoveAssign {
    fn default() -> Self {
        unsafe { NonTrivialMoveAssign { i: 0_() } }
    }
}
pub unsafe fn test_constexpr_move_assignment_non_trivial_29() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable<V>::value");;
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_24(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = Default::default();
        let mut _new_value: () = 101_();
        test_constexpr_assign_imp_24(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: () = Default::default();
        test_constexpr_assign_imp_25(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = 42_();
        let mut _new_value: NonTrivialMoveAssign = NonTrivialMoveAssign::new({ 5_() });
        test_constexpr_assign_imp_26(&mut _v, &mut _new_value)
    });
    (unsafe {
        let mut _v: () = NonTrivialMoveAssign::new({ 3_() });
        let mut _new_value: NonTrivialMoveAssign = NonTrivialMoveAssign::new({ 5_() });
        test_constexpr_assign_imp_26(&mut _v, &mut _new_value)
    });
}
pub unsafe fn non_constexpr_test_30() {
    (unsafe { test_move_assignment_empty_empty_7() });
    (unsafe { test_move_assignment_non_empty_empty_8() });
    (unsafe { test_move_assignment_empty_non_empty_9() });
    (unsafe { test_assignment_throw_20() });
}
pub unsafe fn cxx17_constexpr_test_31() -> () {
    (unsafe { test_move_assignment_sfinae_6() });
    (unsafe { test_move_assignment_noexcept_5() });
    (unsafe { test_constexpr_move_assignment_trivial_27() });
    return true;
}
pub unsafe fn cxx20_constexpr_test_32() -> () {
    (unsafe { test_move_assignment_same_index_10() });
    (unsafe { test_move_assignment_different_index_17() });
    (unsafe { test_constexpr_move_assignment_non_trivial_29() });
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
    (unsafe { non_constexpr_test_30() });
    (unsafe { cxx17_constexpr_test_31() });
    (unsafe { cxx20_constexpr_test_32() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
