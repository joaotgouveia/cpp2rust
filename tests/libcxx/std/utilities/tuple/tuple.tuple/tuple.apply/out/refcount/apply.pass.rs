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
pub fn makeTypeIDImp_2() -> Ptr<TypeID> {
    return id_3.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_4() -> Ptr<TypeID> {
    return id_5.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_6() -> Ptr<TypeID> {
    return id_7.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_8() -> Ptr<TypeID> {
    return id_9.with(|v| v.as_pointer());
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
impl CppEq for TypeID {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
thread_local!(
    static id_3: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<() >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<int &, const int &, unsigned int &>]" )
    })));
);
thread_local!(
    static id_5: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<() >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const int &, const int &, unsigned int &>]" )
    })));
);
thread_local!(
    static id_7: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<() >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<int &&, const int &, unsigned int &&>]" )
    })));
);
thread_local!(
    static id_9: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<() >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const int &&, const int &, unsigned int &&>]" )
    })));
);
pub fn makeArgumentID_10() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_2() });
}
pub fn makeArgumentID_11() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_4() });
}
pub fn makeArgumentID_12() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_6() });
}
pub fn makeArgumentID_13() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_8() });
}
pub fn constexpr_sum_fn_14() -> () {
    return 0_();
}
pub fn constexpr_sum_fn_15(mut x1: ()) -> () {
    return (x1 + ({ constexpr_sum_fn_14() }));
}
pub fn constexpr_sum_fn_16(mut x1: (), mut rest: ()) -> () {
    return (x1 + ({ constexpr_sum_fn_15(rest) }));
}
pub fn constexpr_sum_fn_17(mut x1: (), mut rest: ()) -> () {
    return (x1 + ({ constexpr_sum_fn_15(rest) }));
}
pub fn constexpr_sum_fn_18(mut x1: (), mut rest_1: (), mut rest_2: ()) -> () {
    return (x1 + ({ constexpr_sum_fn_16(rest_1, rest_2) }));
}
pub fn constexpr_sum_fn_19(mut x1: (), mut rest_1: (), mut rest_2: ()) -> () {
    return (x1 + ({ constexpr_sum_fn_16(rest_1, rest_2) }));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConstexprSumT {}
pub fn test_constexpr_evaluation_20() {
    let mut sum_obj: ConstexprSumT = <ConstexprSumT>::default();
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 0"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 0");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 42"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 42");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 143"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 143");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 143"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 143");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 142"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 142");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 142"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 142");;
    }
}
pub type CallQuals = ();
pub const CallQuals_CQ_None: CallQuals = 0;
pub const CallQuals_CQ_LValue: CallQuals = 1;
pub const CallQuals_CQ_ConstLValue: CallQuals = 2;
pub const CallQuals_CQ_RValue: CallQuals = 3;
pub const CallQuals_CQ_ConstRValue: CallQuals = 4;
impl CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    pub fn new(mut q: CallQuals, xargs_1: Ptr<()>, xargs_2: Ptr<()>, xargs_3: Ptr<()>) -> Self {
        Self {
            quals: q,
            arg_types: ({ makeArgumentID_10() }),
            args: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(40)]
pub struct CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    #[offset(0)]
    pub quals: CallQuals,
    #[offset(8)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
    #[offset(16)]
    #[byte_size(24)]
    pub args: (),
}
impl CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    pub fn move_from(
        _a0: Ptr<CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__>,
    ) -> Self {
        Self {
            quals: { (*_a0.upgrade().deref()).quals },
            arg_types: { (*_a0.upgrade().deref()).arg_types.clone() },
            args: unimplemented!(),
        }
    }
}
impl CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    pub fn new(mut q: CallQuals, xargs_1: Ptr<()>, xargs_2: Ptr<()>, xargs_3: Ptr<()>) -> Self {
        Self {
            quals: q,
            arg_types: ({ makeArgumentID_11() }),
            args: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(40)]
pub struct CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    #[offset(0)]
    pub quals: CallQuals,
    #[offset(8)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
    #[offset(16)]
    #[byte_size(24)]
    pub args: (),
}
impl CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    pub fn move_from(
        _a0: Ptr<CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__>,
    ) -> Self {
        Self {
            quals: { (*_a0.upgrade().deref()).quals },
            arg_types: { (*_a0.upgrade().deref()).arg_types.clone() },
            args: unimplemented!(),
        }
    }
}
impl CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    pub fn new(mut q: CallQuals, xargs_1: Ptr<()>, xargs_2: Ptr<()>, xargs_3: Ptr<()>) -> Self {
        Self {
            quals: q,
            arg_types: ({ makeArgumentID_12() }),
            args: unimplemented!(),
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(40)]
pub struct CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    #[offset(0)]
    pub quals: CallQuals,
    #[offset(8)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
    #[offset(16)]
    #[byte_size(24)]
    pub args: (),
}
impl CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    pub fn move_from(
        _a0: Ptr<CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__>,
    ) -> Self {
        Self {
            quals: { (*_a0.upgrade().deref()).quals },
            arg_types: { (*_a0.upgrade().deref()).arg_types.clone() },
            args: unimplemented!(),
        }
    }
}
impl CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    pub fn new(mut q: CallQuals, xargs_1: Ptr<()>, xargs_2: Ptr<()>, xargs_3: Ptr<()>) -> Self {
        Self {
            quals: q,
            arg_types: ({ makeArgumentID_13() }),
            args: unimplemented!(),
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(40)]
pub struct CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    #[offset(0)]
    pub quals: CallQuals,
    #[offset(8)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
    #[offset(16)]
    #[byte_size(24)]
    pub args: (),
}
impl CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    pub fn move_from(
        _a0: Ptr<CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__>,
    ) -> Self {
        Self {
            quals: { (*_a0.upgrade().deref()).quals },
            arg_types: { (*_a0.upgrade().deref()).arg_types.clone() },
            args: unimplemented!(),
        }
    }
}
pub fn makeCallInfo_21(
    mut quals: CallQuals,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    return CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__::new(
        { quals },
        { (args_1).clone() },
        { (args_2).clone() },
        { (args_3).clone() },
    );
}
pub fn makeCallInfo_22(
    mut quals: CallQuals,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    return CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__::new(
        { quals },
        { (args_1).clone() },
        { (args_2).clone() },
        { (args_3).clone() },
    );
}
pub fn makeCallInfo_23(
    mut quals: CallQuals,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    return CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__::new(
        { quals },
        { (args_1).clone() },
        { (args_2).clone() },
        { (args_3).clone() },
    );
}
pub fn makeCallInfo_24(
    mut quals: CallQuals,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    return CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__::new(
        { quals },
        { (args_1).clone() },
        { (args_2).clone() },
        { (args_3).clone() },
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TrackedCallable {}
pub fn check_apply_quals_and_types_25(t: Ptr<()>) {
    let mut expect_args: Ptr<TypeID> = ({ makeArgumentID_10() });
    let obj: Value<TrackedCallable> = Rc::new(RefCell::new(<TrackedCallable>::default()));
    let cobj: Ptr<TrackedCallable> = obj.as_pointer();
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn check_apply_quals_and_types_26(t: Ptr<()>) {
    let mut expect_args: Ptr<TypeID> = ({ makeArgumentID_11() });
    let obj: Value<TrackedCallable> = Rc::new(RefCell::new(<TrackedCallable>::default()));
    let cobj: Ptr<TrackedCallable> = obj.as_pointer();
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn check_apply_quals_and_types_27(t: Ptr<()>) {
    let mut expect_args: Ptr<TypeID> = ({ makeArgumentID_12() });
    let obj: Value<TrackedCallable> = Rc::new(RefCell::new(<TrackedCallable>::default()));
    let cobj: Ptr<TrackedCallable> = obj.as_pointer();
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn check_apply_quals_and_types_28(t: Ptr<()>) {
    let mut expect_args: Ptr<TypeID> = ({ makeArgumentID_13() });
    let obj: Value<TrackedCallable> = Rc::new(RefCell::new(<TrackedCallable>::default()));
    let cobj: Ptr<TrackedCallable> = obj.as_pointer();
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_call_quals_and_arg_types_29() {
    let x: Value<()> = Rc::new(RefCell::new(42_()));
    let y: Value<()> = Rc::new(RefCell::new(101_()));
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ct: Ptr<()> = t.as_pointer();
    ({ check_apply_quals_and_types_25(t.as_pointer()) });
    ({ check_apply_quals_and_types_26((ct).clone()) });
    ({ check_apply_quals_and_types_27(t.as_pointer()) });
    ({ check_apply_quals_and_types_28((ct).clone()) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct NothrowMoveable {}
impl NothrowMoveable {
    pub fn copy_from(_a0: Ptr<NothrowMoveable>) -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<NothrowMoveable>) -> Self {
        Self {}
    }
}
impl Clone for NothrowMoveable {
    fn clone(&self) -> Self {
        let __src: Value<NothrowMoveable> = Rc::new(RefCell::new(NothrowMoveable {}));
        NothrowMoveable::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexceptCallable_true_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexceptCallable_false_ {}
pub fn test_noexcept_30() {
    let mut nec: TestNoexceptCallable_true_ = <TestNoexceptCallable_true_>::default();
    let mut tc: TestNoexceptCallable_false_ = <TestNoexceptCallable_false_>::default();
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
thread_local!(
    pub static my_int_31: Value<()> = Rc::new(RefCell::new(42_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_0_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_1_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_2_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_3_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_4_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_5_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_6_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_7_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_8_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_9_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_10_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_11_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_12_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_13_ {}
pub fn f_32(mut _a0: ReturnTypeTest_index_0_) {}
pub fn f_33(mut _a0: ReturnTypeTest_index_1_) -> () {
    return 0_();
}
pub fn f_34(mut _a0: ReturnTypeTest_index_2_) -> Ptr<()> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_35(mut _a0: ReturnTypeTest_index_3_) -> Ptr<()> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_36(mut _a0: ReturnTypeTest_index_4_) -> Ptr<()> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_37(mut _a0: ReturnTypeTest_index_5_) -> Ptr<()> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_38(mut _a0: ReturnTypeTest_index_6_) -> Ptr<()> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_39(mut _a0: ReturnTypeTest_index_7_) -> Ptr<()> {
    return (my_int_31.with(|v| v.as_pointer())).cast_const();
}
pub fn f_40(mut _a0: ReturnTypeTest_index_8_) -> Ptr<()> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_41(mut _a0: ReturnTypeTest_index_9_) -> Ptr<()> {
    return (my_int_31.with(|v| v.as_pointer())).cast_const();
}
pub fn f_42(mut _a0: ReturnTypeTest_index_10_) -> Ptr<()> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<()>();
}
pub fn f_43(mut _a0: ReturnTypeTest_index_11_) -> Ptr<()> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<()>();
}
pub fn f_44(mut _a0: ReturnTypeTest_index_12_) -> Ptr<()> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<()>();
}
pub fn f_45(mut _a0: ReturnTypeTest_index_13_) -> Ptr<()> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<()>();
}
pub fn test_46() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_0_)> =
        FnPtr::<fn(ReturnTypeTest_index_0_)>::new(f_32);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_47() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_1_) -> ()> =
        FnPtr::<fn(ReturnTypeTest_index_1_) -> ()>::new(f_33);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_48() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_2_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_2_) -> Ptr<()>>::new(f_34);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_49() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_3_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_3_) -> Ptr<()>>::new(f_35);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_50() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_4_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_4_) -> Ptr<()>>::new(f_36);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_51() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_5_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_5_) -> Ptr<()>>::new(f_37);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_52() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_6_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_6_) -> Ptr<()>>::new(f_38);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_53() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_7_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_7_) -> Ptr<()>>::new(f_39);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_54() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_8_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_8_) -> Ptr<()>>::new(f_40);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_55() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_9_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_9_) -> Ptr<()>>::new(f_41);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_56() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_10_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_10_) -> Ptr<()>>::new(f_42);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_57() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_11_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_11_) -> Ptr<()>>::new(f_43);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_58() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_12_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_12_) -> Ptr<()>>::new(f_44);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_59() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: FnPtr<fn(ReturnTypeTest_index_13_) -> Ptr<()>> =
        FnPtr::<fn(ReturnTypeTest_index_13_) -> Ptr<()>>::new(f_45);
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_return_type_60() {
    ({ test_46() });
    ({ test_47() });
    ({ test_48() });
    ({ test_49() });
    ({ test_50() });
    ({ test_51() });
    ({ test_52() });
    ({ test_53() });
    ({ test_54() });
    ({ test_55() });
    ({ test_56() });
    ({ test_57() });
    ({ test_58() });
    ({ test_59() });
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
    ({ test_constexpr_evaluation_20() });
    ({ test_call_quals_and_arg_types_29() });
    ({ test_return_type_60() });
    ({ test_noexcept_30() });
    return 0_();
}
pub trait ConstexprSumTImpl {
    fn operator_call_2(&self) -> ();
    fn operator_call_2_int(&self, values: ()) -> ();
    fn operator_call_2_int_long(&self, values_0: (), values_1: ()) -> ();
    fn operator_call_2_int_long_int(&self, values_0: (), values_1: (), values_2: ()) -> ();
    fn operator_call_2_int_int_int(&self, values_0: (), values_1: (), values_2: ()) -> ();
}
impl ConstexprSumTImpl for Ptr<ConstexprSumT> {
    fn operator_call_2(&self) -> () {
        return ({ constexpr_sum_fn_14() });
    }
    fn operator_call_2_int(&self, mut values: ()) -> () {
        return ({ constexpr_sum_fn_15(values) });
    }
    fn operator_call_2_int_long(&self, mut values_0: (), mut values_1: ()) -> () {
        return ({ constexpr_sum_fn_17(values_0, values_1) });
    }
    fn operator_call_2_int_long_int(
        &self,
        mut values_0: (),
        mut values_1: (),
        mut values_2: (),
    ) -> () {
        return ({ constexpr_sum_fn_19(values_0, values_1, values_2) });
    }
    fn operator_call_2_int_int_int(
        &self,
        mut values_0: (),
        mut values_1: (),
        mut values_2: (),
    ) -> () {
        return ({ constexpr_sum_fn_18(values_0, values_1, values_2) });
    }
}
pub trait TestNoexceptCallable_false_Impl {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        _a0: (),
        _a1: Ptr<()>,
        _a2: (),
    ) -> NothrowMoveable;
}
impl TestNoexceptCallable_false_Impl for Ptr<TestNoexceptCallable_false_> {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        mut _a0: (),
        mut _a1: Ptr<()>,
        mut _a2: (),
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
}
pub trait TestNoexceptCallable_true_Impl {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        _a0: (),
        _a1: Ptr<()>,
        _a2: (),
    ) -> NothrowMoveable;
    fn operator_call_1_struct_NothrowMoveable_int(
        &self,
        _a0: NothrowMoveable,
        _a1: (),
    ) -> NothrowMoveable;
}
impl TestNoexceptCallable_true_Impl for Ptr<TestNoexceptCallable_true_> {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        mut _a0: (),
        mut _a1: Ptr<()>,
        mut _a2: (),
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
    fn operator_call_1_struct_NothrowMoveable_int(
        &self,
        mut _a0: NothrowMoveable,
        mut _a1: (),
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
}
pub trait TrackedCallableImpl {
    fn operator_call_2_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_2_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_2_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_2_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_3_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_3_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_3_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_3_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_4_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_4_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_4_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_4_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_5_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_5_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_5_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_5_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
}
impl TrackedCallableImpl for Ptr<TrackedCallable> {
    fn operator_call_2_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_2_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_2_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_2_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<()>,
        xargs_1: Ptr<()>,
        xargs_2: Ptr<()>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<()> = (xargs_0).clone();
            let _args_2: Ptr<()> = (xargs_1).clone();
            let _args_3: Ptr<()> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
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
    let _ = my_int_31.with(|_| ());
}
