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
pub type CallType = u32;
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_2(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as u32) | (RHS as u32)) as CallType);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(1)]
pub struct ForwardingCallObject {}
impl CppMove for ForwardingCallObject {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static last_call_type_3: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_4: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
impl CppMove for ReturnFirst {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
impl CppMove for ReturnArity {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCopy {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyOnly {}
impl CppMove for CopyOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveOnly {}
impl MoveOnly {
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {}
    }
}
impl CppMove for MoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveOnlyNT {}
impl MoveOnlyNT {
    pub fn move_from(_a0: Ptr<MoveOnlyNT>) -> Self {
        Self {}
    }
}
impl CppMove for MoveOnlyNT {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveOnlyOddNothrow {}
impl MoveOnlyOddNothrow {
    pub fn move_from(_a0: Ptr<MoveOnlyOddNothrow>) -> Self {
        Self {}
    }
}
impl CppMove for MoveOnlyOddNothrow {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveAssignOnly {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MoveAssign {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub move_construct: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign: Ptr<i32>,
}
impl MoveAssign {
    pub fn new(mut v: i32, mut move_ctor: Ptr<i32>, mut move_assi: Ptr<i32>) -> Self {
        Self {
            value: v,
            move_construct: (move_ctor).clone(),
            move_assign: (move_assi).clone(),
        }
    }
    pub fn move_from(o: Ptr<MoveAssign>) -> Self {
        let __this: Value<MoveAssign> = Rc::new(RefCell::new(Self {
            value: o.with(|__s| __s.value),
            move_construct: o.with(|__s| __s.move_construct.clone()),
            move_assign: o.with(|__s| __s.move_assign.clone()),
        }));
        let this: Ptr<MoveAssign> = __this.as_pointer();
        this.with(|__s| __s.move_construct.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        field!(o, value).write(-1_i32);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppMove for MoveAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NTMoveAssign {
    #[offset(0)]
    pub value: i32,
}
impl NTMoveAssign {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppMove for NTMoveAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_move_assignable<NTMoveAssign>::value"
);
const _: () = assert!(true, "std::is_move_assignable<NTMoveAssign>::value");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct TMoveAssign {
    #[offset(0)]
    pub value: i32,
}
impl TMoveAssign {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
    pub fn move_from(_a0: Ptr<TMoveAssign>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
impl CppMove for TMoveAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_assignable<TMoveAssign>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct TMoveAssignNTCopyAssign {
    #[offset(0)]
    pub value: i32,
}
impl TMoveAssignNTCopyAssign {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppMove for TMoveAssignNTCopyAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_assignable_v<TMoveAssignNTCopyAssign>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct TrivialCopyNontrivialMove {}
impl TrivialCopyNontrivialMove {
    pub fn move_from(_a0: Ptr<TrivialCopyNontrivialMove>) -> Self {
        Self {}
    }
}
impl CppMove for TrivialCopyNontrivialMove {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
pub fn test_move_assignment_noexcept_5() {
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
pub fn test_move_assignment_sfinae_6() {
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
pub fn test_move_assignment_empty_empty_7() {}
pub fn test_move_assignment_non_empty_empty_8() {}
pub fn test_move_assignment_empty_non_empty_9() {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Result_int_ {
    #[offset(0)]
    pub index: usize,
    #[offset(8)]
    pub value: i32,
}
impl CppMove for Result_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Result_long_ {
    #[offset(0)]
    pub index: usize,
    #[offset(8)]
    pub value: i64,
}
impl CppMove for Result_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_move_assignment_same_index_10() {
    {
        let v1: Value<std_variant_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int_> = Rc::new(RefCell::new(42));
        let vref: Ptr<std_variant_int_> = ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 0_usize) }));
        assert!(((({ get_11(v1.as_pointer(),) }).read()) == 42));
    }
    {
        let v1: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43_i64));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        let vref: Ptr<std_variant_int__long__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!(((({ get_12(v1.as_pointer(),) }).read()) == 42_i64));
    }
    {
        let move_construct: Value<i32> = Rc::new(RefCell::new(0));
        let move_assign: Value<i32> = Rc::new(RefCell::new(0));
        let v1: Value<std_variant_int__MoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(43));
            let __tmp_1: Value<Ptr<i32>> = Rc::new(RefCell::new((move_construct.as_pointer())));
            let __tmp_2: Value<Ptr<i32>> = Rc::new(RefCell::new((move_assign.as_pointer())));
            std_variant_int__MoveAssign__unsigned_int_::new_1(
                { (*in_place_type_13.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
                { __tmp_1.as_pointer() },
                { __tmp_2.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__MoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_4: Value<Ptr<i32>> = Rc::new(RefCell::new((move_construct.as_pointer())));
            let __tmp_5: Value<Ptr<i32>> = Rc::new(RefCell::new((move_assign.as_pointer())));
            std_variant_int__MoveAssign__unsigned_int_::new_1(
                { (*in_place_type_13.with(Value::clone).borrow()).clone() },
                { __tmp_3.as_pointer() },
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
            )
        }));
        let vref: Ptr<std_variant_int__MoveAssign__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!((({ get_14(v1.as_pointer(),) }).with(|__s| __s.value) == 42));
        assert!(({ ((*move_construct.borrow()) == 0) }));
        assert!(({ ((*move_assign.borrow()) == 1) }));
    }
    {
        let test: Value<anon_15> = Rc::new(RefCell::new(<anon_15>::default()));
        let mut result: Result_int_ = ({ anon_15Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_16> = Rc::new(RefCell::new(<anon_16>::default()));
        let mut result: Result_long_ = ({ anon_16Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_17> = Rc::new(RefCell::new(<anon_17>::default()));
        let mut result: Result_int_ = ({ anon_17Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_15 {}
impl Callable0<Result_int_> for anon_15 {
    fn call(&self) -> Result_int_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_15Impl::operator_call(__it))
    }
}
impl CppMove for anon_15 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_16 {}
impl Callable0<Result_long_> for anon_16 {
    fn call(&self) -> Result_long_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_16Impl::operator_call(__it))
    }
}
impl CppMove for anon_16 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_17 {}
impl Callable0<Result_int_> for anon_17 {
    fn call(&self) -> Result_int_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_17Impl::operator_call(__it))
    }
}
impl CppMove for anon_17 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_move_assignment_different_index_20() {
    {
        let v1: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        let vref: Ptr<std_variant_int__long__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!(((({ get_12(v1.as_pointer(),) }).read()) == 42_i64));
    }
    {
        let move_construct: Value<i32> = Rc::new(RefCell::new(0));
        let move_assign: Value<i32> = Rc::new(RefCell::new(0));
        let v1: Value<std_variant_int__MoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_8: Value<u32> = Rc::new(RefCell::new(43_u32));
            std_variant_int__MoveAssign__unsigned_int_::new_2(
                { (*in_place_type_21.with(Value::clone).borrow()).clone() },
                { __tmp_8.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__MoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_9: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_10: Value<Ptr<i32>> = Rc::new(RefCell::new((move_construct.as_pointer())));
            let __tmp_11: Value<Ptr<i32>> = Rc::new(RefCell::new((move_assign.as_pointer())));
            std_variant_int__MoveAssign__unsigned_int_::new_1(
                { (*in_place_type_13.with(Value::clone).borrow()).clone() },
                { __tmp_9.as_pointer() },
                { __tmp_10.as_pointer() },
                { __tmp_11.as_pointer() },
            )
        }));
        let vref: Ptr<std_variant_int__MoveAssign__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!((({ get_14(v1.as_pointer(),) }).with(|__s| __s.value) == 42));
        assert!(({ ((*move_construct.borrow()) == 1) }));
        assert!(({ ((*move_assign.borrow()) == 0) }));
    }
    {
        let test: Value<anon_22> = Rc::new(RefCell::new(<anon_22>::default()));
        let mut result: Result_long_ = ({ anon_22Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_23> = Rc::new(RefCell::new(<anon_23>::default()));
        let mut result: Result_long_ = ({ anon_23Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_22 {}
impl Callable0<Result_long_> for anon_22 {
    fn call(&self) -> Result_long_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_22Impl::operator_call(__it))
    }
}
impl CppMove for anon_22 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_23 {}
impl Callable0<Result_long_> for anon_23 {
    fn call(&self) -> Result_long_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_23Impl::operator_call(__it))
    }
}
impl CppMove for anon_23 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_assignment_throw_24() {}
pub fn test_constexpr_move_assignment_trivial_25() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(42_i64));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_26(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(Default::default()));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_26(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(42_i64));
        let _new_value: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_27(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(42_i64));
        let _new_value: Value<i32> = Rc::new(RefCell::new(101));
        test_constexpr_assign_imp_28(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn operator_eq_29(x: Ptr<NonTrivialMoveAssign>, y: Ptr<NonTrivialMoveAssign>) -> bool {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NonTrivialMoveAssign {
    #[offset(0)]
    pub i: i32,
}
impl NonTrivialMoveAssign {
    pub fn new(mut ii: i32) -> Self {
        Self { i: ii }
    }
    pub fn move_from(other: Ptr<NonTrivialMoveAssign>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
}
impl CppEq for NonTrivialMoveAssign {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_29(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for NonTrivialMoveAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for NonTrivialMoveAssign {
    fn default() -> Self {
        { NonTrivialMoveAssign { i: 0 } }
    }
}
pub fn test_constexpr_move_assignment_non_trivial_30() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable<V>::value");;
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialMoveAssign_> =
            Rc::new(RefCell::new(42_i64));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_31(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialMoveAssign_> =
            Rc::new(RefCell::new(Default::default()));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_31(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialMoveAssign_> =
            Rc::new(RefCell::new(42_i64));
        let _new_value: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_32(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialMoveAssign_> =
            Rc::new(RefCell::new(42_i64));
        let _new_value: Value<NonTrivialMoveAssign> =
            Rc::new(RefCell::new(NonTrivialMoveAssign::new({ 5 })));
        test_constexpr_assign_imp_33(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialMoveAssign_> =
            Rc::new(RefCell::new(NonTrivialMoveAssign::new({ 3 })));
        let _new_value: Value<NonTrivialMoveAssign> =
            Rc::new(RefCell::new(NonTrivialMoveAssign::new({ 5 })));
        test_constexpr_assign_imp_33(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn non_constexpr_test_34() {
    ({ test_move_assignment_empty_empty_7() });
    ({ test_move_assignment_non_empty_empty_8() });
    ({ test_move_assignment_empty_non_empty_9() });
    ({ test_assignment_throw_24() });
}
pub fn cxx17_constexpr_test_35() -> bool {
    ({ test_move_assignment_sfinae_6() });
    ({ test_move_assignment_noexcept_5() });
    ({ test_constexpr_move_assignment_trivial_25() });
    return true;
}
pub fn cxx20_constexpr_test_36() -> bool {
    ({ test_move_assignment_same_index_10() });
    ({ test_move_assignment_different_index_20() });
    ({ test_constexpr_move_assignment_non_trivial_30() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ non_constexpr_test_34() });
    ({ cxx17_constexpr_test_35() });
    ({ cxx20_constexpr_test_36() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0;
}
pub trait ForwardingCallObjectImpl {
    fn to_i32(&self) -> i32;
}
impl ForwardingCallObjectImpl for Ptr<ForwardingCallObject> {
    fn to_i32(&self) -> i32 {
        return 0;
    }
}
pub trait MoveAssignImpl {
    fn move_assign(&self, o: Ptr<MoveAssign>) -> Ptr<MoveAssign>;
}
impl MoveAssignImpl for Ptr<MoveAssign> {
    fn move_assign(&self, o: Ptr<MoveAssign>) -> Ptr<MoveAssign> {
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!((*self), move_construct).write({ o.with(|__s| __s.move_construct.clone()) });
        field!((*self), move_assign).write({ o.with(|__s| __s.move_assign.clone()) });
        (*self).with(|__s| __s.move_assign.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        field!(o, value).write(-1_i32);
        return (*self).clone();
    }
}
pub trait MoveAssignOnlyImpl {
    fn move_assign(&self, _a0: Ptr<MoveAssignOnly>) -> Ptr<MoveAssignOnly>;
}
impl MoveAssignOnlyImpl for Ptr<MoveAssignOnly> {
    fn move_assign(&self, _a0: Ptr<MoveAssignOnly>) -> Ptr<MoveAssignOnly> {
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        return (*self).clone();
    }
}
pub trait MoveOnlyNTImpl {
    fn move_assign(&self, _a0: Ptr<MoveOnlyNT>) -> Ptr<MoveOnlyNT>;
}
impl MoveOnlyNTImpl for Ptr<MoveOnlyNT> {
    fn move_assign(&self, _a0: Ptr<MoveOnlyNT>) -> Ptr<MoveOnlyNT> {
        return (*self).clone();
    }
}
pub trait MoveOnlyOddNothrowImpl {
    fn move_assign(&self, _a0: Ptr<MoveOnlyOddNothrow>) -> Ptr<MoveOnlyOddNothrow>;
}
impl MoveOnlyOddNothrowImpl for Ptr<MoveOnlyOddNothrow> {
    fn move_assign(&self, _a0: Ptr<MoveOnlyOddNothrow>) -> Ptr<MoveOnlyOddNothrow> {
        return (*self).clone();
    }
}
pub trait NTMoveAssignImpl {
    fn move_assign(&self, that: Ptr<NTMoveAssign>) -> Ptr<NTMoveAssign>;
}
impl NTMoveAssignImpl for Ptr<NTMoveAssign> {
    fn move_assign(&self, that: Ptr<NTMoveAssign>) -> Ptr<NTMoveAssign> {
        field!((*self), value).write({ that.with(|__s| __s.value) });
        field!(that, value).write(-1_i32);
        return (*self).clone();
    }
}
pub trait NonTrivialMoveAssignImpl {
    fn move_assign(&self, o: Ptr<NonTrivialMoveAssign>) -> Ptr<NonTrivialMoveAssign>;
}
impl NonTrivialMoveAssignImpl for Ptr<NonTrivialMoveAssign> {
    fn move_assign(&self, o: Ptr<NonTrivialMoveAssign>) -> Ptr<NonTrivialMoveAssign> {
        field!((*self), i).write({ o.with(|__s| __s.i) });
        return (*self).clone();
    }
}
pub trait TMoveAssignImpl {
    fn move_assign(&self, _a0: Ptr<TMoveAssign>) -> Ptr<TMoveAssign>;
}
impl TMoveAssignImpl for Ptr<TMoveAssign> {
    fn move_assign(&self, _a0: Ptr<TMoveAssign>) -> Ptr<TMoveAssign> {
        field!((*self), value).write({ { (*_a0.upgrade().deref()).value } });
        return (*self).clone();
    }
}
pub trait TMoveAssignNTCopyAssignImpl {
    fn copy_assign(&self, that: Ptr<TMoveAssignNTCopyAssign>) -> Ptr<TMoveAssignNTCopyAssign>;
    fn move_assign(&self, _a0: Ptr<TMoveAssignNTCopyAssign>) -> Ptr<TMoveAssignNTCopyAssign>;
}
impl TMoveAssignNTCopyAssignImpl for Ptr<TMoveAssignNTCopyAssign> {
    fn copy_assign(&self, that: Ptr<TMoveAssignNTCopyAssign>) -> Ptr<TMoveAssignNTCopyAssign> {
        field!((*self), value).write({ that.with(|__s| __s.value) });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<TMoveAssignNTCopyAssign>) -> Ptr<TMoveAssignNTCopyAssign> {
        field!((*self), value).write({ { (*_a0.upgrade().deref()).value } });
        return (*self).clone();
    }
}
pub trait TrivialCopyNontrivialMoveImpl {
    fn move_assign(&self, _a0: Ptr<TrivialCopyNontrivialMove>) -> Ptr<TrivialCopyNontrivialMove>;
}
impl TrivialCopyNontrivialMoveImpl for Ptr<TrivialCopyNontrivialMove> {
    fn move_assign(&self, _a0: Ptr<TrivialCopyNontrivialMove>) -> Ptr<TrivialCopyNontrivialMove> {
        return (*self).clone();
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
pub trait anon_15Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_15Impl for Ptr<anon_15> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<std_variant_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int_> = Rc::new(RefCell::new(42));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_int_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_11(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_16Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_16Impl for Ptr<anon_16> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43_i64));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_long_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_12(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_17Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_17Impl for Ptr<anon_17> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<std_variant_int__TMoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_6: Value<i32> = Rc::new(RefCell::new(43));
            std_variant_int__TMoveAssign__unsigned_int_::new_1(
                { (*in_place_type_18.with(Value::clone).borrow()).clone() },
                { __tmp_6.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__TMoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_7: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__TMoveAssign__unsigned_int_::new_1(
                { (*in_place_type_18.with(Value::clone).borrow()).clone() },
                { __tmp_7.as_pointer() },
            )
        }));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_int_ {
            index: ({ (*v.borrow()).index() }),
            value: ({ get_19(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_22Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_22Impl for Ptr<anon_22> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_long_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_12(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_23Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_23Impl for Ptr<anon_23> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<std_variant_int__TMoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_12: Value<u32> = Rc::new(RefCell::new(43_u32));
            std_variant_int__TMoveAssign__unsigned_int_::new_2(
                { (*in_place_type_21.with(Value::clone).borrow()).clone() },
                { __tmp_12.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__TMoveAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_13: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__TMoveAssign__unsigned_int_::new_1(
                { (*in_place_type_18.with(Value::clone).borrow()).clone() },
                { __tmp_13.as_pointer() },
            )
        }));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_long_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_19(v.as_pointer()) }).with(|__s| __s.value) as i64),
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_3.with(|_| ());
    let _ = last_call_args_4.with(|_| ());
}
