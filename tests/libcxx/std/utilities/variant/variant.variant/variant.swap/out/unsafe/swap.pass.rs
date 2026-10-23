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
pub struct NotSwappable {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotCopyable {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotCopyableWithSwap {}
pub unsafe fn swap_5(_a0: *mut NotCopyableWithSwap, mut _a1: NotCopyableWithSwap) {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NotMoveAssignable {}
impl NotMoveAssignable {
    pub unsafe fn move_from(_a0: *mut NotMoveAssignable) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NotMoveAssignableWithSwap {}
impl NotMoveAssignableWithSwap {
    pub unsafe fn move_from(_a0: *mut NotMoveAssignableWithSwap) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn swap_6(_a0: *mut NotMoveAssignableWithSwap, _a1: *mut NotMoveAssignableWithSwap) {}
pub unsafe fn do_throw_7() {}
pub unsafe fn do_throw_8() {
    unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NothrowTypeImp_false__true__false__true__false__false_ {
    pub value: (),
    pub move_called: *mut (),
    pub move_assign_called: *mut (),
    pub swap_called: *mut (),
}
impl NothrowTypeImp_false__true__false__true__false__false_ {
    pub unsafe fn new(
        mut v: (),
        mut mv_ctr: *mut (),
        mut mv_assign: *mut (),
        mut swap: *mut (),
    ) -> Self {
        let mut this = Self {
            value: v,
            move_called: mv_ctr,
            move_assign_called: mv_assign,
            swap_called: swap,
        };
        this
    }
    pub unsafe fn move_from(
        o: *mut NothrowTypeImp_false__true__false__true__false__false_,
    ) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_called: (*o).move_called,
            move_assign_called: (*o).move_assign_called,
            swap_called: (*o).swap_called,
        };
        (*this.move_called).prefix_inc();
        (unsafe { do_throw_7() });
        (*o).value = -1_();
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut NothrowTypeImp_false__true__false__true__false__false_,
    ) -> *mut NothrowTypeImp_false__true__false__true__false__false_ {
        (*self.move_assign_called).prefix_inc();
        (unsafe { do_throw_7() });
        self.value = (*o).value;
        (*o).value = -1_();
        return &mut (*(self as *mut NothrowTypeImp_false__true__false__true__false__false_));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NothrowTypeImp_false__true__false__false__false__false_ {
    pub value: (),
    pub move_called: *mut (),
    pub move_assign_called: *mut (),
    pub swap_called: *mut (),
}
impl NothrowTypeImp_false__true__false__false__false__false_ {
    pub unsafe fn move_from(
        o: *mut NothrowTypeImp_false__true__false__false__false__false_,
    ) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_called: (*o).move_called,
            move_assign_called: (*o).move_assign_called,
            swap_called: (*o).swap_called,
        };
        (*this.move_called).prefix_inc();
        (unsafe { do_throw_7() });
        (*o).value = -1_();
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        o: *mut NothrowTypeImp_false__true__false__false__false__false_,
    ) -> *mut NothrowTypeImp_false__true__false__false__false__false_ {
        (*self.move_assign_called).prefix_inc();
        (unsafe { do_throw_8() });
        self.value = (*o).value;
        (*o).value = -1_();
        return &mut (*(self as *mut NothrowTypeImp_false__true__false__false__false__false_));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NothrowTypeImp_false__true__false__false__false__true_ {
    pub value: (),
    pub move_called: *mut (),
    pub move_assign_called: *mut (),
    pub swap_called: *mut (),
}
impl NothrowTypeImp_false__true__false__false__false__true_ {
    pub unsafe fn new(
        mut v: (),
        mut mv_ctr: *mut (),
        mut mv_assign: *mut (),
        mut swap: *mut (),
    ) -> Self {
        let mut this = Self {
            value: v,
            move_called: mv_ctr,
            move_assign_called: mv_assign,
            swap_called: swap,
        };
        this
    }
    pub unsafe fn move_from(
        o: *mut NothrowTypeImp_false__true__false__false__false__true_,
    ) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_called: (*o).move_called,
            move_assign_called: (*o).move_assign_called,
            swap_called: (*o).swap_called,
        };
        (*this.move_called).prefix_inc();
        (unsafe { do_throw_7() });
        (*o).value = -1_();
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NothrowTypeImp_false__false__false__false__true__true_ {
    pub value: (),
    pub move_called: *mut (),
    pub move_assign_called: *mut (),
    pub swap_called: *mut (),
}
impl NothrowTypeImp_false__false__false__false__true__true_ {
    pub unsafe fn new(
        mut v: (),
        mut mv_ctr: *mut (),
        mut mv_assign: *mut (),
        mut swap: *mut (),
    ) -> Self {
        let mut this = Self {
            value: v,
            move_called: mv_ctr,
            move_assign_called: mv_assign,
            swap_called: swap,
        };
        this
    }
    pub unsafe fn move_from(
        o: *mut NothrowTypeImp_false__false__false__false__true__true_,
    ) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_called: (*o).move_called,
            move_assign_called: (*o).move_assign_called,
            swap_called: (*o).swap_called,
        };
        (*this.move_called).prefix_inc();
        (unsafe { do_throw_8() });
        (*o).value = -1_();
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NothrowTypeImp_false__true__false__false__true__true_ {
    pub value: (),
    pub move_called: *mut (),
    pub move_assign_called: *mut (),
    pub swap_called: *mut (),
}
impl NothrowTypeImp_false__true__false__false__true__true_ {
    pub unsafe fn move_from(o: *mut NothrowTypeImp_false__true__false__false__true__true_) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_called: (*o).move_called,
            move_assign_called: (*o).move_assign_called,
            swap_called: (*o).swap_called,
        };
        (*this.move_called).prefix_inc();
        (unsafe { do_throw_7() });
        (*o).value = -1_();
        this
    }
}
pub unsafe fn swap_9(
    lhs: *mut NothrowTypeImp_false__false__false__false__true__true_,
    rhs: *mut NothrowTypeImp_false__false__false__false__true__true_,
) {
    (*(*lhs).swap_called).prefix_inc();
    (unsafe { do_throw_7() });
    (unsafe {
        let ___x: *mut () = &mut (*lhs).value;
        let ___y: *mut () = &mut (*rhs).value;
        swap_10(___x, ___y)
    });
}
pub unsafe fn swap_11(
    lhs: *mut NothrowTypeImp_false__true__false__false__false__true_,
    rhs: *mut NothrowTypeImp_false__true__false__false__false__true_,
) {
    (*(*lhs).swap_called).prefix_inc();
    (unsafe { do_throw_8() });
    (unsafe {
        let ___x: *mut () = &mut (*lhs).value;
        let ___y: *mut () = &mut (*rhs).value;
        swap_10(___x, ___y)
    });
}
pub unsafe fn swap_12(
    lhs: *mut NothrowTypeImp_false__true__false__false__true__true_,
    rhs: *mut NothrowTypeImp_false__true__false__false__true__true_,
) {
    (*(*lhs).swap_called).prefix_inc();
    (unsafe { do_throw_7() });
    (unsafe {
        let ___x: *mut () = &mut (*lhs).value;
        let ___y: *mut () = &mut (*rhs).value;
        swap_10(___x, ___y)
    });
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NonThrowingNonNoexceptType {
    pub value: (),
    pub move_called: *mut (),
}
impl NonThrowingNonNoexceptType {
    pub unsafe fn new(mut v: (), mut mv_called: *mut ()) -> Self {
        let mut this = Self {
            value: v,
            move_called: mv_called,
        };
        this
    }
    pub unsafe fn move_from(o: *mut NonThrowingNonNoexceptType) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_called: (*o).move_called,
        };
        (*this.move_called).prefix_inc();
        (*o).value = -1_();
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut NonThrowingNonNoexceptType,
    ) -> *mut NonThrowingNonNoexceptType {
        unimplemented!();
        return &mut (*(self as *mut NonThrowingNonNoexceptType));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct ThrowsOnSecondMove {
    pub value: (),
    pub move_count: (),
}
impl ThrowsOnSecondMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            value: v,
            move_count: 0_(),
        };
        this
    }
    pub unsafe fn move_from(o: *mut ThrowsOnSecondMove) -> Self {
        let mut this = Self {
            value: (*o).value,
            move_count: (((*o).move_count) + (1_())),
        };
        if ((this.move_count) == (2_())) {
            (unsafe { do_throw_8() });
        }
        (*o).value = -1_();
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut ThrowsOnSecondMove) -> *mut ThrowsOnSecondMove {
        unimplemented!();
        return &mut (*(self as *mut ThrowsOnSecondMove));
    }
}
pub unsafe fn test_swap_valueless_by_exception_13() {}
pub unsafe fn test_swap_same_alternative_14() {
    {
        let mut move_called: () = 0_();
        let mut move_assign_called: () = 0_();
        let mut swap_called: () = 0_();
        let mut v1: () = {
            let mut __tmp_0: () = 42_();
            let mut __tmp_1: *mut () = (&mut move_called as *mut ());
            let mut __tmp_2: *mut () = (&mut move_assign_called as *mut ());
            let mut __tmp_3: *mut () = (&mut swap_called as *mut ());
            std_variant_NothrowTypeImp_false__false__false__false__true__true___int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_15)).clone() },
                { &mut __tmp_0 },
                { &mut __tmp_1 },
                { &mut __tmp_2 },
                { &mut __tmp_3 },
            )
        };
        let mut v2: () = {
            let mut __tmp_4: () = 100_();
            let mut __tmp_5: *mut () = (&mut move_called as *mut ());
            let mut __tmp_6: *mut () = (&mut move_assign_called as *mut ());
            let mut __tmp_7: *mut () = (&mut swap_called as *mut ());
            std_variant_NothrowTypeImp_false__false__false__false__true__true___int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_15)).clone() },
                { &mut __tmp_4 },
                { &mut __tmp_5 },
                { &mut __tmp_6 },
                { &mut __tmp_7 },
            )
        };
        unimplemented!();
        unimplemented!();
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
        let mut move_called: () = 0_();
        let mut move_assign_called: () = 0_();
        let mut swap_called: () = 0_();
        let mut v1: () = {
            let mut __tmp_8: () = 42_();
            let mut __tmp_9: *mut () = (&mut move_called as *mut ());
            let mut __tmp_10: *mut () = (&mut move_assign_called as *mut ());
            let mut __tmp_11: *mut () = (&mut swap_called as *mut ());
            std_variant_NothrowTypeImp_false__true__false__true__false__false___int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_15)).clone() },
                { &mut __tmp_8 },
                { &mut __tmp_9 },
                { &mut __tmp_10 },
                { &mut __tmp_11 },
            )
        };
        let mut v2: () = {
            let mut __tmp_12: () = 100_();
            let mut __tmp_13: *mut () = (&mut move_called as *mut ());
            let mut __tmp_14: *mut () = (&mut move_assign_called as *mut ());
            let mut __tmp_15: *mut () = (&mut swap_called as *mut ());
            std_variant_NothrowTypeImp_false__true__false__true__false__false___int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_15)).clone() },
                { &mut __tmp_12 },
                { &mut __tmp_13 },
                { &mut __tmp_14 },
                { &mut __tmp_15 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        move_called = 0_();
        move_assign_called = 0_();
        swap_called = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_swap_same_alternative_throws_16() {}
pub unsafe fn test_swap_different_alternatives_17() {
    {
        let mut move_called: () = 0_();
        let mut move_assign_called: () = 0_();
        let mut swap_called: () = 0_();
        let mut v1: () = {
            let mut __tmp_16: () = 42_();
            let mut __tmp_17: *mut () = (&mut move_called as *mut ());
            let mut __tmp_18: *mut () = (&mut move_assign_called as *mut ());
            let mut __tmp_19: *mut () = (&mut swap_called as *mut ());
            std_variant_NothrowTypeImp_false__true__false__false__false__true___int_::new_1(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_15)).clone() },
                { &mut __tmp_16 },
                { &mut __tmp_17 },
                { &mut __tmp_18 },
                { &mut __tmp_19 },
            )
        };
        let mut v2: () = {
            let mut __tmp_20: () = 100_();
            std_variant_NothrowTypeImp_false__true__false__false__false__true___int_::new_2(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_18)).clone() },
                { &mut __tmp_20 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        move_called = 0_();
        move_assign_called = 0_();
        swap_called = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_swap_different_alternatives_throws_19() {}
pub unsafe fn has_swap_member_imp_20(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_swap_member_imp_21(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_swap_member_imp_22(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_swap_member_imp_23(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_swap_member_imp_24(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn has_swap_member_imp_25(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn has_swap_member_imp_26(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn has_swap_member_imp_27(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn has_swap_member_imp_28(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn has_swap_member_29() -> () {
    return (unsafe { has_swap_member_imp_25(0_()) });
}
pub unsafe fn has_swap_member_30() -> () {
    return (unsafe { has_swap_member_imp_26(0_()) });
}
pub unsafe fn has_swap_member_31() -> () {
    return (unsafe { has_swap_member_imp_27(0_()) });
}
pub unsafe fn has_swap_member_32() -> () {
    return (unsafe { has_swap_member_imp_28(0_()) });
}
pub unsafe fn has_swap_member_33() -> () {
    return (unsafe { has_swap_member_imp_20(0_()) });
}
pub unsafe fn has_swap_member_34() -> () {
    return (unsafe { has_swap_member_imp_21(0_()) });
}
pub unsafe fn has_swap_member_35() -> () {
    return (unsafe { has_swap_member_imp_22(0_()) });
}
pub unsafe fn has_swap_member_36() -> () {
    return (unsafe { has_swap_member_imp_23(0_()) });
}
pub unsafe fn has_swap_member_37() -> () {
    return (unsafe { has_swap_member_imp_24(0_()) });
}
pub unsafe fn test_swap_sfinae_38() {
    {
        const _: () = assert!(true, "!has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_swappable_v<V>");;
    }
    {
        const _: () = assert!(true, "!has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_swappable_v<V>");;
    }
    {
        const _: () = assert!(true, "!has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_swappable_v<V>");;
    }
    {
        const _: () = assert!(true, "!has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_swappable_v<V>");;
    }
}
pub unsafe fn test_swap_noexcept_39() {
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_swappable_v<V>");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        (unsafe { swap_40(&mut v1, &mut v2) });
    }
}
pub unsafe fn non_constexpr_test_41() {
    (unsafe { test_swap_valueless_by_exception_13() });
    (unsafe { test_swap_same_alternative_throws_16() });
    (unsafe { test_swap_different_alternatives_throws_19() });
}
pub unsafe fn test_42() -> () {
    (unsafe { test_swap_same_alternative_14() });
    (unsafe { test_swap_different_alternatives_17() });
    (unsafe { test_swap_sfinae_38() });
    (unsafe { test_swap_noexcept_39() });
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
    (unsafe { non_constexpr_test_41() });
    (unsafe { test_42() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const last_call_type_3);
    std::cell::LazyCell::force(&*&raw const last_call_args_4);
}
