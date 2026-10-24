extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_1(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct TypeID {
    #[offset(0)]
    #[byte_size(8)]
    m_id: Ptr<()>,
}
impl TypeID {
    fn new(mut xid: Ptr<()>) -> Self {
        Self {
            m_id: (xid).clone(),
        }
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(TypeID {
                    m_id: self.m_id.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TypeID {
                    m_id: other.m_id.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TypeID {}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_2(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as ()) | (RHS as ())) as CallType);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardingCallObject {}
thread_local!(
    pub static last_call_type_3: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_4: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotSwappable {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotCopyable {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotCopyableWithSwap {}
pub fn swap_5(_a0: Ptr<NotCopyableWithSwap>, mut _a1: NotCopyableWithSwap) {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct NotMoveAssignable {}
impl NotMoveAssignable {
    pub fn move_from(_a0: Ptr<NotMoveAssignable>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct NotMoveAssignableWithSwap {}
impl NotMoveAssignableWithSwap {
    pub fn move_from(_a0: Ptr<NotMoveAssignableWithSwap>) -> Self {
        Self {}
    }
}
pub fn swap_6(_a0: Ptr<NotMoveAssignableWithSwap>, _a1: Ptr<NotMoveAssignableWithSwap>) {}
pub fn do_throw_7() {}
pub fn do_throw_8() {
    unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct NothrowTypeImp_false__true__false__true__false__false_ {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_called: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign_called: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub swap_called: Ptr<()>,
}
impl NothrowTypeImp_false__true__false__true__false__false_ {
    pub fn new(mut v: (), mut mv_ctr: Ptr<()>, mut mv_assign: Ptr<()>, mut swap: Ptr<()>) -> Self {
        Self {
            value: v,
            move_called: (mv_ctr).clone(),
            move_assign_called: (mv_assign).clone(),
            swap_called: (swap).clone(),
        }
    }
    pub fn move_from(o: Ptr<NothrowTypeImp_false__true__false__true__false__false_>) -> Self {
        let __this: Value<NothrowTypeImp_false__true__false__true__false__false_> =
            Rc::new(RefCell::new(Self {
                value: o.with(|__s| __s.value),
                move_called: o.with(|__s| __s.move_called.clone()),
                move_assign_called: o.with(|__s| __s.move_assign_called.clone()),
                swap_called: o.with(|__s| __s.swap_called.clone()),
            }));
        let this: Ptr<NothrowTypeImp_false__true__false__true__false__false_> = __this.as_pointer();
        this.with(|__s| __s.move_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_7() });
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct NothrowTypeImp_false__true__false__false__false__false_ {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_called: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign_called: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub swap_called: Ptr<()>,
}
impl NothrowTypeImp_false__true__false__false__false__false_ {
    pub fn move_from(o: Ptr<NothrowTypeImp_false__true__false__false__false__false_>) -> Self {
        let __this: Value<NothrowTypeImp_false__true__false__false__false__false_> =
            Rc::new(RefCell::new(Self {
                value: o.with(|__s| __s.value),
                move_called: o.with(|__s| __s.move_called.clone()),
                move_assign_called: o.with(|__s| __s.move_assign_called.clone()),
                swap_called: o.with(|__s| __s.swap_called.clone()),
            }));
        let this: Ptr<NothrowTypeImp_false__true__false__false__false__false_> =
            __this.as_pointer();
        this.with(|__s| __s.move_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_7() });
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(32)]
pub struct NothrowTypeImp_false__true__false__false__false__true_ {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_called: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign_called: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub swap_called: Ptr<()>,
}
impl NothrowTypeImp_false__true__false__false__false__true_ {
    pub fn new(mut v: (), mut mv_ctr: Ptr<()>, mut mv_assign: Ptr<()>, mut swap: Ptr<()>) -> Self {
        Self {
            value: v,
            move_called: (mv_ctr).clone(),
            move_assign_called: (mv_assign).clone(),
            swap_called: (swap).clone(),
        }
    }
    pub fn move_from(o: Ptr<NothrowTypeImp_false__true__false__false__false__true_>) -> Self {
        let __this: Value<NothrowTypeImp_false__true__false__false__false__true_> =
            Rc::new(RefCell::new(Self {
                value: o.with(|__s| __s.value),
                move_called: o.with(|__s| __s.move_called.clone()),
                move_assign_called: o.with(|__s| __s.move_assign_called.clone()),
                swap_called: o.with(|__s| __s.swap_called.clone()),
            }));
        let this: Ptr<NothrowTypeImp_false__true__false__false__false__true_> = __this.as_pointer();
        this.with(|__s| __s.move_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_7() });
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(32)]
pub struct NothrowTypeImp_false__false__false__false__true__true_ {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_called: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign_called: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub swap_called: Ptr<()>,
}
impl NothrowTypeImp_false__false__false__false__true__true_ {
    pub fn new(mut v: (), mut mv_ctr: Ptr<()>, mut mv_assign: Ptr<()>, mut swap: Ptr<()>) -> Self {
        Self {
            value: v,
            move_called: (mv_ctr).clone(),
            move_assign_called: (mv_assign).clone(),
            swap_called: (swap).clone(),
        }
    }
    pub fn move_from(o: Ptr<NothrowTypeImp_false__false__false__false__true__true_>) -> Self {
        let __this: Value<NothrowTypeImp_false__false__false__false__true__true_> =
            Rc::new(RefCell::new(Self {
                value: o.with(|__s| __s.value),
                move_called: o.with(|__s| __s.move_called.clone()),
                move_assign_called: o.with(|__s| __s.move_assign_called.clone()),
                swap_called: o.with(|__s| __s.swap_called.clone()),
            }));
        let this: Ptr<NothrowTypeImp_false__false__false__false__true__true_> = __this.as_pointer();
        this.with(|__s| __s.move_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_8() });
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(32)]
pub struct NothrowTypeImp_false__true__false__false__true__true_ {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_called: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign_called: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub swap_called: Ptr<()>,
}
impl NothrowTypeImp_false__true__false__false__true__true_ {
    pub fn move_from(o: Ptr<NothrowTypeImp_false__true__false__false__true__true_>) -> Self {
        let __this: Value<NothrowTypeImp_false__true__false__false__true__true_> =
            Rc::new(RefCell::new(Self {
                value: o.with(|__s| __s.value),
                move_called: o.with(|__s| __s.move_called.clone()),
                move_assign_called: o.with(|__s| __s.move_assign_called.clone()),
                swap_called: o.with(|__s| __s.swap_called.clone()),
            }));
        let this: Ptr<NothrowTypeImp_false__true__false__false__true__true_> = __this.as_pointer();
        this.with(|__s| __s.move_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_7() });
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn swap_9(
    lhs: Ptr<NothrowTypeImp_false__false__false__false__true__true_>,
    rhs: Ptr<NothrowTypeImp_false__false__false__false__true__true_>,
) {
    lhs.with(|__s| __s.swap_called.clone())
        .with_mut(|__v| __v.prefix_inc());
    ({ do_throw_7() });
    unimplemented!();
}
pub fn swap_10(
    lhs: Ptr<NothrowTypeImp_false__true__false__false__false__true_>,
    rhs: Ptr<NothrowTypeImp_false__true__false__false__false__true_>,
) {
    lhs.with(|__s| __s.swap_called.clone())
        .with_mut(|__v| __v.prefix_inc());
    ({ do_throw_8() });
    unimplemented!();
}
pub fn swap_11(
    lhs: Ptr<NothrowTypeImp_false__true__false__false__true__true_>,
    rhs: Ptr<NothrowTypeImp_false__true__false__false__true__true_>,
) {
    lhs.with(|__s| __s.swap_called.clone())
        .with_mut(|__v| __v.prefix_inc());
    ({ do_throw_7() });
    unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct NonThrowingNonNoexceptType {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_called: Ptr<()>,
}
impl NonThrowingNonNoexceptType {
    pub fn new(mut v: (), mut mv_called: Ptr<()>) -> Self {
        Self {
            value: v,
            move_called: (mv_called).clone(),
        }
    }
    pub fn move_from(o: Ptr<NonThrowingNonNoexceptType>) -> Self {
        let __this: Value<NonThrowingNonNoexceptType> = Rc::new(RefCell::new(Self {
            value: o.with(|__s| __s.value),
            move_called: o.with(|__s| __s.move_called.clone()),
        }));
        let this: Ptr<NonThrowingNonNoexceptType> = __this.as_pointer();
        this.with(|__s| __s.move_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct ThrowsOnSecondMove {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    pub move_count: (),
}
impl ThrowsOnSecondMove {
    pub fn new(mut v: ()) -> Self {
        Self {
            value: v,
            move_count: 0_(),
        }
    }
    pub fn move_from(o: Ptr<ThrowsOnSecondMove>) -> Self {
        let __this: Value<ThrowsOnSecondMove> = Rc::new(RefCell::new(Self {
            value: o.with(|__s| __s.value),
            move_count: (o.with(|__s| __s.move_count) + 1_()),
        }));
        let this: Ptr<ThrowsOnSecondMove> = __this.as_pointer();
        if (this.with(|__s| __s.move_count) == 2_()) {
            ({ do_throw_8() });
        }
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn test_swap_valueless_by_exception_12() {}
pub fn test_swap_same_alternative_13() {
    {
        let move_called: Value<()> = Rc::new(RefCell::new(0_()));
        let move_assign_called: Value<()> = Rc::new(RefCell::new(0_()));
        let swap_called: Value<()> = Rc::new(RefCell::new(0_()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
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
        let move_called: Value<()> = Rc::new(RefCell::new(0_()));
        let move_assign_called: Value<()> = Rc::new(RefCell::new(0_()));
        let swap_called: Value<()> = Rc::new(RefCell::new(0_()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*move_called.borrow_mut()) = 0_();
        (*move_assign_called.borrow_mut()) = 0_();
        (*swap_called.borrow_mut()) = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_swap_same_alternative_throws_14() {}
pub fn test_swap_different_alternatives_15() {
    {
        let move_called: Value<()> = Rc::new(RefCell::new(0_()));
        let move_assign_called: Value<()> = Rc::new(RefCell::new(0_()));
        let swap_called: Value<()> = Rc::new(RefCell::new(0_()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*move_called.borrow_mut()) = 0_();
        (*move_assign_called.borrow_mut()) = 0_();
        (*swap_called.borrow_mut()) = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_swap_different_alternatives_throws_16() {}
pub fn has_swap_member_imp_17(mut _a0: ()) -> () {
    return true;
}
pub fn has_swap_member_imp_18(mut _a0: ()) -> () {
    return true;
}
pub fn has_swap_member_imp_19(mut _a0: ()) -> () {
    return true;
}
pub fn has_swap_member_imp_20(mut _a0: ()) -> () {
    return true;
}
pub fn has_swap_member_imp_21(mut _a0: ()) -> () {
    return true;
}
pub fn has_swap_member_imp_22(mut _a0: ()) -> () {
    return false;
}
pub fn has_swap_member_imp_23(mut _a0: ()) -> () {
    return false;
}
pub fn has_swap_member_imp_24(mut _a0: ()) -> () {
    return false;
}
pub fn has_swap_member_imp_25(mut _a0: ()) -> () {
    return false;
}
pub fn has_swap_member_26() -> () {
    return ({ has_swap_member_imp_22(0_()) });
}
pub fn has_swap_member_27() -> () {
    return ({ has_swap_member_imp_23(0_()) });
}
pub fn has_swap_member_28() -> () {
    return ({ has_swap_member_imp_24(0_()) });
}
pub fn has_swap_member_29() -> () {
    return ({ has_swap_member_imp_25(0_()) });
}
pub fn has_swap_member_30() -> () {
    return ({ has_swap_member_imp_17(0_()) });
}
pub fn has_swap_member_31() -> () {
    return ({ has_swap_member_imp_18(0_()) });
}
pub fn has_swap_member_32() -> () {
    return ({ has_swap_member_imp_19(0_()) });
}
pub fn has_swap_member_33() -> () {
    return ({ has_swap_member_imp_20(0_()) });
}
pub fn has_swap_member_34() -> () {
    return ({ has_swap_member_imp_21(0_()) });
}
pub fn test_swap_sfinae_35() {
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
pub fn test_swap_noexcept_36() {
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "std::is_swappable_v<V> && has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ (*v1.borrow()).swap(v2.as_pointer()) });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!has_swap_member<V>()");;
        const _: () = assert!(true, "std::is_swappable_v<V>");;
        const _: () = assert!(true, "std::is_nothrow_swappable_v<V>");;
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn non_constexpr_test_37() {
    ({ test_swap_valueless_by_exception_12() });
    ({ test_swap_same_alternative_throws_14() });
    ({ test_swap_different_alternatives_throws_16() });
}
pub fn test_38() -> () {
    ({ test_swap_same_alternative_13() });
    ({ test_swap_different_alternatives_15() });
    ({ test_swap_sfinae_35() });
    ({ test_swap_noexcept_36() });
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ non_constexpr_test_37() });
    ({ test_38() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait ForwardingCallObjectImpl {
    fn to_(&self) -> ();
}
impl ForwardingCallObjectImpl for Ptr<ForwardingCallObject> {
    fn to_(&self) -> () {
        return 0_();
    }
}
pub trait NonThrowingNonNoexceptTypeImpl {
    fn move_assign(&self, _a0: Ptr<NonThrowingNonNoexceptType>) -> Ptr<NonThrowingNonNoexceptType>;
}
impl NonThrowingNonNoexceptTypeImpl for Ptr<NonThrowingNonNoexceptType> {
    fn move_assign(&self, _a0: Ptr<NonThrowingNonNoexceptType>) -> Ptr<NonThrowingNonNoexceptType> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait NothrowTypeImp_false__false__false__false__true__true_Impl {
    fn copy_assign(
        &self,
        _a0: Ptr<NothrowTypeImp_false__false__false__false__true__true_>,
    ) -> Ptr<NothrowTypeImp_false__false__false__false__true__true_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__false__false__false__true__true_>,
    ) -> Ptr<NothrowTypeImp_false__false__false__false__true__true_> {
        unimplemented!()
    }
}
impl NothrowTypeImp_false__false__false__false__true__true_Impl
    for Ptr<NothrowTypeImp_false__false__false__false__true__true_>
{
}
pub trait NothrowTypeImp_false__true__false__false__false__false_Impl {
    fn copy_assign(
        &self,
        _a0: Ptr<NothrowTypeImp_false__true__false__false__false__false_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__false__false_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__true__false__false__false__false_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__false__false_>;
}
impl NothrowTypeImp_false__true__false__false__false__false_Impl
    for Ptr<NothrowTypeImp_false__true__false__false__false__false_>
{
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__true__false__false__false__false_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__false__false_> {
        (*self)
            .with(|__s| __s.move_assign_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_8() });
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!(o, value).write(-1_());
        return (*self).clone();
    }
}
pub trait NothrowTypeImp_false__true__false__false__false__true_Impl {
    fn copy_assign(
        &self,
        _a0: Ptr<NothrowTypeImp_false__true__false__false__false__true_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__false__true_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__true__false__false__false__true_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__false__true_> {
        unimplemented!()
    }
}
impl NothrowTypeImp_false__true__false__false__false__true_Impl
    for Ptr<NothrowTypeImp_false__true__false__false__false__true_>
{
}
pub trait NothrowTypeImp_false__true__false__false__true__true_Impl {
    fn copy_assign(
        &self,
        _a0: Ptr<NothrowTypeImp_false__true__false__false__true__true_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__true__true_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__true__false__false__true__true_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__false__true__true_> {
        unimplemented!()
    }
}
impl NothrowTypeImp_false__true__false__false__true__true_Impl
    for Ptr<NothrowTypeImp_false__true__false__false__true__true_>
{
}
pub trait NothrowTypeImp_false__true__false__true__false__false_Impl {
    fn copy_assign(
        &self,
        _a0: Ptr<NothrowTypeImp_false__true__false__true__false__false_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__true__false__false_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__true__false__true__false__false_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__true__false__false_>;
}
impl NothrowTypeImp_false__true__false__true__false__false_Impl
    for Ptr<NothrowTypeImp_false__true__false__true__false__false_>
{
    fn move_assign(
        &self,
        o: Ptr<NothrowTypeImp_false__true__false__true__false__false_>,
    ) -> Ptr<NothrowTypeImp_false__true__false__true__false__false_> {
        (*self)
            .with(|__s| __s.move_assign_called.clone())
            .with_mut(|__v| __v.prefix_inc());
        ({ do_throw_7() });
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!(o, value).write(-1_());
        return (*self).clone();
    }
}
pub trait ThrowsOnSecondMoveImpl {
    fn move_assign(&self, _a0: Ptr<ThrowsOnSecondMove>) -> Ptr<ThrowsOnSecondMove>;
}
impl ThrowsOnSecondMoveImpl for Ptr<ThrowsOnSecondMove> {
    fn move_assign(&self, _a0: Ptr<ThrowsOnSecondMove>) -> Ptr<ThrowsOnSecondMove> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait TypeIDImpl {
    fn name(&self) -> ();
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_3.with(|_| ());
    let _ = last_call_args_4.with(|_| ());
}
