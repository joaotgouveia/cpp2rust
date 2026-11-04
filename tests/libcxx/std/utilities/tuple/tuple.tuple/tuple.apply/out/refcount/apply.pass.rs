extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> bool {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_1(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> bool {
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
    m_id: Ptr<i8>,
}
impl TypeID {
    fn new(mut xid: Ptr<i8>) -> Self {
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
        Ptr::<i8 >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<int &, const int &, unsigned int &>]" )
    })));
);
thread_local!(
    static id_5: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<i8 >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const int &, const int &, unsigned int &>]" )
    })));
);
thread_local!(
    static id_7: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<i8 >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<int &&, const int &, unsigned int &&>]" )
    })));
);
thread_local!(
    static id_9: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<i8 >::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const int &&, const int &, unsigned int &&>]" )
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
pub fn constexpr_sum_fn_14() -> i32 {
    return 0;
}
pub fn constexpr_sum_fn_15(mut x1: i32) -> i32 {
    return (x1 + ({ constexpr_sum_fn_14() }));
}
pub fn constexpr_sum_fn_16(mut x1: i32, mut rest: i32) -> i32 {
    return (x1 + ({ constexpr_sum_fn_15(rest) }));
}
pub fn constexpr_sum_fn_17(mut x1: i32, mut rest: i64) -> i32 {
    return (x1 + ({ constexpr_sum_fn_15((rest as i32)) }));
}
pub fn constexpr_sum_fn_18(mut x1: i32, mut rest_1: i32, mut rest_2: i32) -> i32 {
    return (x1 + ({ constexpr_sum_fn_16(rest_1, rest_2) }));
}
pub fn constexpr_sum_fn_19(mut x1: i32, mut rest_1: i64, mut rest_2: i32) -> i32 {
    return (x1 + ({ constexpr_sum_fn_16((rest_1 as i32), rest_2) }));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConstexprSumT {}
impl Callable0<i32> for ConstexprSumT {
    fn call(&self) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            ConstexprSumTImpl::operator_call_2(__it)
        })
    }
}
impl Callable1<i32, i32> for ConstexprSumT {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            ConstexprSumTImpl::operator_call_2_int(__it, a1)
        })
    }
}
impl Callable2<i32, i64, i32> for ConstexprSumT {
    fn call(&self, a1: i32, a2: i64) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            ConstexprSumTImpl::operator_call_2_int_long(__it, a1, a2)
        })
    }
}
impl Callable3<i32, i64, i32, i32> for ConstexprSumT {
    fn call(&self, a1: i32, a2: i64, a3: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            ConstexprSumTImpl::operator_call_2_int_long_int(__it, a1, a2, a3)
        })
    }
}
impl Callable3<i32, i32, i32, i32> for ConstexprSumT {
    fn call(&self, a1: i32, a2: i32, a3: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            ConstexprSumTImpl::operator_call_2_int_int_int(__it, a1, a2, a3)
        })
    }
}
impl CppMove for ConstexprSumT {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_constexpr_evaluation_20() {
    let sum_obj: Value<ConstexprSumT> = Rc::new(RefCell::new(<ConstexprSumT>::default()));
    {
        let t: Value<std_tuple__> = Rc::new(RefCell::new(<std_tuple__>::default()));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 0"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 0");;
    }
    {
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(42));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 42"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 42");;
    }
    {
        let t: Value<std_tuple_int__long_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(101));
            std_tuple_int__long_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 143"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 143");;
    }
    {
        let t: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(42)),
            Rc::new(RefCell::new((101 as i64))),
        )));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 143"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 143");;
    }
    {
        let t: Value<std_tuple_int__long__int_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(101));
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(-1_i32));
            std_tuple_int__long__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() }, {
                __tmp_4.as_pointer()
            })
        }));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 142"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 142");;
    }
    {
        let t: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([42, 101, -1_i32])));
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 142"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 142");;
    }
}
pub type CallQuals = u32;
pub const CallQuals_CQ_None: CallQuals = 0;
pub const CallQuals_CQ_LValue: CallQuals = 1;
pub const CallQuals_CQ_ConstLValue: CallQuals = 2;
pub const CallQuals_CQ_RValue: CallQuals = 3;
pub const CallQuals_CQ_ConstRValue: CallQuals = 4;
pub fn makeCallInfo_21(
    mut quals: CallQuals,
    args_1: Ptr<i32>,
    args_2: Ptr<i32>,
    args_3: Ptr<u32>,
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
    args_1: Ptr<i32>,
    args_2: Ptr<i32>,
    args_3: Ptr<u32>,
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
    args_1: Ptr<i32>,
    args_2: Ptr<i32>,
    args_3: Ptr<u32>,
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
    args_1: Ptr<i32>,
    args_2: Ptr<i32>,
    args_3: Ptr<u32>,
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
impl
    Callable3<
        Ptr<i32>,
        Ptr<i32>,
        Ptr<u32>,
        CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__,
    > for TrackedCallable
{
    fn call(
        &self,
        a1: Ptr<i32>,
        a2: Ptr<i32>,
        a3: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TrackedCallableImpl::operator_call_2_int_ref_const_int_ref_unsigned_int_ref(
                __it, a1, a2, a3,
            )
        })
    }
}
impl
    Callable3<
        Ptr<i32>,
        Ptr<i32>,
        Ptr<u32>,
        CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__,
    > for TrackedCallable
{
    fn call(
        &self,
        a1: Ptr<i32>,
        a2: Ptr<i32>,
        a3: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TrackedCallableImpl::operator_call_2_const_int_ref_const_int_ref_unsigned_int_ref(
                __it, a1, a2, a3,
            )
        })
    }
}
impl
    Callable3<
        Ptr<i32>,
        Ptr<i32>,
        Ptr<u32>,
        CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__,
    > for TrackedCallable
{
    fn call(
        &self,
        a1: Ptr<i32>,
        a2: Ptr<i32>,
        a3: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TrackedCallableImpl::operator_call_2_int_const_int_ref_unsigned_int(__it, a1, a2, a3)
        })
    }
}
impl
    Callable3<
        Ptr<i32>,
        Ptr<i32>,
        Ptr<u32>,
        CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__,
    > for TrackedCallable
{
    fn call(
        &self,
        a1: Ptr<i32>,
        a2: Ptr<i32>,
        a3: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TrackedCallableImpl::operator_call_2_const_int_const_int_ref_unsigned_int(
                __it, a1, a2, a3,
            )
        })
    }
}
impl CppMove for TrackedCallable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_call_quals_and_arg_types_25() {
    let x: Value<i32> = Rc::new(RefCell::new(42));
    let y: Value<u32> = Rc::new(RefCell::new(101_u32));
    let t: Value<std_tuple_int__const_int_ref__unsigned_int_refref_> = Rc::new(RefCell::new({
        let __tmp_5: Value<i32> = Rc::new(RefCell::new(-1_i32));
        std_tuple_int__const_int_ref__unsigned_int_refref_::new_1(
            { __tmp_5.as_pointer() },
            { x.as_pointer() },
            { y.as_pointer() },
        )
    }));
    let ct: Ptr<std_tuple_int__const_int_ref__unsigned_int_refref_> = t.as_pointer();
    ({ check_apply_quals_and_types_26(t.as_pointer()) });
    ({ check_apply_quals_and_types_27((ct).clone()) });
    ({ check_apply_quals_and_types_28(t.as_pointer()) });
    ({ check_apply_quals_and_types_29((ct).clone()) });
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
impl CppMove for NothrowMoveable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexceptCallable_true_ {}
impl Callable3<i32, Ptr<i8>, i64, NothrowMoveable> for TestNoexceptCallable_true_ {
    fn call(&self, a1: i32, a2: Ptr<i8>, a3: i64) -> NothrowMoveable {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestNoexceptCallable_true_Impl::operator_call_1_int_const_char_ptr_long(
                __it, a1, a2, a3,
            )
        })
    }
}
impl Callable2<NothrowMoveable, i32, NothrowMoveable> for TestNoexceptCallable_true_ {
    fn call(&self, a1: NothrowMoveable, a2: i32) -> NothrowMoveable {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestNoexceptCallable_true_Impl::operator_call_1_struct_NothrowMoveable_int(__it, a1, a2)
        })
    }
}
impl CppMove for TestNoexceptCallable_true_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexceptCallable_false_ {}
impl Callable3<i32, Ptr<i8>, i64, NothrowMoveable> for TestNoexceptCallable_false_ {
    fn call(&self, a1: i32, a2: Ptr<i8>, a3: i64) -> NothrowMoveable {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestNoexceptCallable_false_Impl::operator_call_1_int_const_char_ptr_long(
                __it, a1, a2, a3,
            )
        })
    }
}
impl CppMove for TestNoexceptCallable_false_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_noexcept_30() {
    let nec: Value<TestNoexceptCallable_true_> =
        Rc::new(RefCell::new(<TestNoexceptCallable_true_>::default()));
    let tc: Value<TestNoexceptCallable_false_> =
        Rc::new(RefCell::new(<TestNoexceptCallable_false_>::default()));
    {
        let t: Value<std_tuple_int__const_char_ptr__long_> = Rc::new(RefCell::new(
            <std_tuple_int__const_char_ptr__long_>::default(),
        ));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let t: Value<std_tuple_NothrowMoveable__int_> =
            Rc::new(RefCell::new(<std_tuple_NothrowMoveable__int_>::default()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
thread_local!(
    pub static my_int_31: Value<i32> = Rc::new(RefCell::new(42));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_0_ {}
impl CppMove for ReturnTypeTest_index_0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_1_ {}
impl CppMove for ReturnTypeTest_index_1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_2_ {}
impl CppMove for ReturnTypeTest_index_2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_3_ {}
impl CppMove for ReturnTypeTest_index_3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_4_ {}
impl CppMove for ReturnTypeTest_index_4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_5_ {}
impl CppMove for ReturnTypeTest_index_5_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_6_ {}
impl CppMove for ReturnTypeTest_index_6_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_7_ {}
impl CppMove for ReturnTypeTest_index_7_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_8_ {}
impl CppMove for ReturnTypeTest_index_8_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_9_ {}
impl CppMove for ReturnTypeTest_index_9_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_10_ {}
impl CppMove for ReturnTypeTest_index_10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_11_ {}
impl CppMove for ReturnTypeTest_index_11_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_12_ {}
impl CppMove for ReturnTypeTest_index_12_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnTypeTest_index_13_ {}
impl CppMove for ReturnTypeTest_index_13_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn f_32(mut _a0: ReturnTypeTest_index_0_) {}
pub fn f_33(mut _a0: ReturnTypeTest_index_1_) -> i32 {
    return 0;
}
pub fn f_34(mut _a0: ReturnTypeTest_index_2_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_35(mut _a0: ReturnTypeTest_index_3_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_36(mut _a0: ReturnTypeTest_index_4_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_37(mut _a0: ReturnTypeTest_index_5_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_38(mut _a0: ReturnTypeTest_index_6_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_39(mut _a0: ReturnTypeTest_index_7_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_40(mut _a0: ReturnTypeTest_index_8_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_41(mut _a0: ReturnTypeTest_index_9_) -> Ptr<i32> {
    return my_int_31.with(|v| v.as_pointer());
}
pub fn f_42(mut _a0: ReturnTypeTest_index_10_) -> Ptr<i32> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<i32>();
}
pub fn f_43(mut _a0: ReturnTypeTest_index_11_) -> Ptr<i32> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<i32>();
}
pub fn f_44(mut _a0: ReturnTypeTest_index_12_) -> Ptr<i32> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<i32>();
}
pub fn f_45(mut _a0: ReturnTypeTest_index_13_) -> Ptr<i32> {
    return (my_int_31.with(|v| v.as_pointer())).reinterpret_cast::<i32>();
}
pub fn test_46() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_0_)>> = Rc::new(RefCell::new(FnPtr::<
        fn(ReturnTypeTest_index_0_),
    >::new(f_32)));
    let t: Value<std_tuple_ReturnTypeTest_index_0__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_0__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_47() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_1_) -> i32>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(ReturnTypeTest_index_1_) -> i32>::new(f_33),
        ));
    let t: Value<std_tuple_ReturnTypeTest_index_1__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_1__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_48() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_2_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_2_) -> Ptr<i32>>::new(f_34),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_2__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_2__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_49() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_3_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_3_) -> Ptr<i32>>::new(f_35),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_3__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_3__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_50() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_4_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_4_) -> Ptr<i32>>::new(f_36),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_4__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_4__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_51() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_5_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_5_) -> Ptr<i32>>::new(f_37),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_5__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_5__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_52() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_6_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_6_) -> Ptr<i32>>::new(f_38),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_6__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_6__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_53() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_7_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_7_) -> Ptr<i32>>::new(f_39),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_7__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_7__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_54() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_8_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_8_) -> Ptr<i32>>::new(f_40),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_8__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_8__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_55() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_9_) -> Ptr<i32>>> = Rc::new(RefCell::new(
        FnPtr::<fn(ReturnTypeTest_index_9_) -> Ptr<i32>>::new(f_41),
    ));
    let t: Value<std_tuple_ReturnTypeTest_index_9__> =
        Rc::new(RefCell::new(<std_tuple_ReturnTypeTest_index_9__>::default()));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_56() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_10_) -> Ptr<i32>>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(ReturnTypeTest_index_10_) -> Ptr<i32>,
        >::new(f_42)));
    let t: Value<std_tuple_ReturnTypeTest_index_10__> = Rc::new(RefCell::new(
        <std_tuple_ReturnTypeTest_index_10__>::default(),
    ));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_57() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_11_) -> Ptr<i32>>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(ReturnTypeTest_index_11_) -> Ptr<i32>,
        >::new(f_43)));
    let t: Value<std_tuple_ReturnTypeTest_index_11__> = Rc::new(RefCell::new(
        <std_tuple_ReturnTypeTest_index_11__>::default(),
    ));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_58() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_12_) -> Ptr<i32>>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(ReturnTypeTest_index_12_) -> Ptr<i32>,
        >::new(f_44)));
    let t: Value<std_tuple_ReturnTypeTest_index_12__> = Rc::new(RefCell::new(
        <std_tuple_ReturnTypeTest_index_12__>::default(),
    ));
    (&(*t.borrow_mut()));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub fn test_59() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let fn_: Value<FnPtr<fn(ReturnTypeTest_index_13_) -> Ptr<i32>>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(ReturnTypeTest_index_13_) -> Ptr<i32>,
        >::new(f_45)));
    let t: Value<std_tuple_ReturnTypeTest_index_13__> = Rc::new(RefCell::new(
        <std_tuple_ReturnTypeTest_index_13__>::default(),
    ));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_constexpr_evaluation_20() });
    ({ test_call_quals_and_arg_types_25() });
    ({ test_return_type_60() });
    ({ test_noexcept_30() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
pub trait ConstexprSumTImpl {
    fn operator_call_2(&self) -> i32;
    fn operator_call_2_int(&self, values: i32) -> i32;
    fn operator_call_2_int_long(&self, values_0: i32, values_1: i64) -> i32;
    fn operator_call_2_int_long_int(&self, values_0: i32, values_1: i64, values_2: i32) -> i32;
    fn operator_call_2_int_int_int(&self, values_0: i32, values_1: i32, values_2: i32) -> i32;
}
impl ConstexprSumTImpl for Ptr<ConstexprSumT> {
    fn operator_call_2(&self) -> i32 {
        return ({ constexpr_sum_fn_14() });
    }
    fn operator_call_2_int(&self, mut values: i32) -> i32 {
        return ({ constexpr_sum_fn_15(values) });
    }
    fn operator_call_2_int_long(&self, mut values_0: i32, mut values_1: i64) -> i32 {
        return ({ constexpr_sum_fn_17(values_0, values_1) });
    }
    fn operator_call_2_int_long_int(
        &self,
        mut values_0: i32,
        mut values_1: i64,
        mut values_2: i32,
    ) -> i32 {
        return ({ constexpr_sum_fn_19(values_0, values_1, values_2) });
    }
    fn operator_call_2_int_int_int(
        &self,
        mut values_0: i32,
        mut values_1: i32,
        mut values_2: i32,
    ) -> i32 {
        return ({ constexpr_sum_fn_18(values_0, values_1, values_2) });
    }
}
pub trait TestNoexceptCallable_false_Impl {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        _a0: i32,
        _a1: Ptr<i8>,
        _a2: i64,
    ) -> NothrowMoveable;
}
impl TestNoexceptCallable_false_Impl for Ptr<TestNoexceptCallable_false_> {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        mut _a0: i32,
        mut _a1: Ptr<i8>,
        mut _a2: i64,
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
}
pub trait TestNoexceptCallable_true_Impl {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        _a0: i32,
        _a1: Ptr<i8>,
        _a2: i64,
    ) -> NothrowMoveable;
    fn operator_call_1_struct_NothrowMoveable_int(
        &self,
        _a0: NothrowMoveable,
        _a1: i32,
    ) -> NothrowMoveable;
}
impl TestNoexceptCallable_true_Impl for Ptr<TestNoexceptCallable_true_> {
    fn operator_call_1_int_const_char_ptr_long(
        &self,
        mut _a0: i32,
        mut _a1: Ptr<i8>,
        mut _a2: i64,
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
    fn operator_call_1_struct_NothrowMoveable_int(
        &self,
        mut _a0: NothrowMoveable,
        mut _a1: i32,
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
}
pub trait TrackedCallableImpl {
    fn operator_call_2_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_2_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_2_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_2_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_3_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_3_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_3_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_3_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_4_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_4_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_4_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_4_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_5_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_5_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__;
    fn operator_call_5_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__;
    fn operator_call_5_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__;
}
impl TrackedCallableImpl for Ptr<TrackedCallable> {
    fn operator_call_2_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_2_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_2_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_2_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_3_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_4_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_21(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_22(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_23(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    fn operator_call_5_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: Ptr<i32>,
        xargs_1: Ptr<i32>,
        xargs_2: Ptr<u32>,
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return ({
            let _args_1: Ptr<i32> = (xargs_0).clone();
            let _args_2: Ptr<i32> = (xargs_1).clone();
            let _args_3: Ptr<u32> = (xargs_2).clone();
            makeCallInfo_24(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
}
pub trait TypeIDImpl {
    fn name(&self) -> Vec<i8>;
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> Vec<i8> {
        return {
            let mut __bytes = (*self).with(|__s| __s.m_id.clone()).to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = my_int_31.with(|_| ());
}
