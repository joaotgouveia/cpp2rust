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
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCopy {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyOnly {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveOnly {}
impl MoveOnly {
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {}
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveOnlyOddNothrow {}
impl MoveOnlyOddNothrow {
    pub fn move_from(_a0: Ptr<MoveOnlyOddNothrow>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveAssignOnly {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct MoveAssign {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub move_construct: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub move_assign: Ptr<()>,
}
impl MoveAssign {
    pub fn new(mut v: (), mut move_ctor: Ptr<()>, mut move_assi: Ptr<()>) -> Self {
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
        this.with(|__s| __s.move_construct.clone())
            .with_mut(|__v| __v.prefix_inc());
        field!(o, value).write(-1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveAssign, Default)]
#[byte_size(4)]
pub struct NTMoveAssign {
    #[offset(0)]
    pub value: (),
}
impl NTMoveAssign {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_move_assignable<NTMoveAssign>::value"
);
const _: () = assert!(true, "std::is_move_assignable<NTMoveAssign>::value");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(4)]
pub struct TMoveAssign {
    #[offset(0)]
    pub value: (),
}
impl TMoveAssign {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn move_from(_a0: Ptr<TMoveAssign>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_assignable<TMoveAssign>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(4)]
pub struct TMoveAssignNTCopyAssign {
    #[offset(0)]
    pub value: (),
}
impl TMoveAssignNTCopyAssign {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
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
    pub index: (),
    #[offset(8)]
    pub value: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Result_long_ {
    #[offset(0)]
    pub index: (),
    #[offset(8)]
    pub value: (),
}
pub fn test_move_assignment_same_index_10() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let move_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let move_assign: Value<()> = Rc::new(RefCell::new(0_()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let test: Value<anon_11> = Rc::new(RefCell::new(<anon_11>::default()));
        let mut result: Result_int_ = ({ anon_11Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_13> = Rc::new(RefCell::new(<anon_13>::default()));
        let mut result: Result_long_ = ({ anon_13Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_15> = Rc::new(RefCell::new(<anon_15>::default()));
        let mut result: Result_int_ = ({ anon_15Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_11 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_13 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_15 {}
pub fn test_move_assignment_different_index_17() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let move_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let move_assign: Value<()> = Rc::new(RefCell::new(0_()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let test: Value<anon_18> = Rc::new(RefCell::new(<anon_18>::default()));
        let mut result: Result_long_ = ({ anon_18Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_19> = Rc::new(RefCell::new(<anon_19>::default()));
        let mut result: Result_long_ = ({ anon_19Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_18 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_19 {}
pub fn test_assignment_throw_20() {}
pub fn test_constexpr_assign_imp_21(v: Ptr<()>, new_value: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_22(v: Ptr<()>, new_value: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_23(v: Ptr<()>, new_value: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_24(v: Ptr<()>, new_value: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_25(v: Ptr<()>, new_value: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_26(v: Ptr<()>, new_value: Ptr<NonTrivialMoveAssign>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_move_assignment_trivial_27() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_21(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_21(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_22(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_23(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn operator_eq_28(x: Ptr<NonTrivialMoveAssign>, y: Ptr<NonTrivialMoveAssign>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct NonTrivialMoveAssign {
    #[offset(0)]
    pub i: (),
}
impl NonTrivialMoveAssign {
    pub fn new(mut ii: ()) -> Self {
        Self { i: ii }
    }
    pub fn move_from(other: Ptr<NonTrivialMoveAssign>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
}
impl std::cmp::PartialEq for NonTrivialMoveAssign {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_28(
                Rc::new(RefCell::new(NonTrivialMoveAssign { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(NonTrivialMoveAssign { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialMoveAssign {}
impl Default for NonTrivialMoveAssign {
    fn default() -> Self {
        { NonTrivialMoveAssign { i: 0_() } }
    }
}
pub fn test_constexpr_move_assignment_non_trivial_29() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_24(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_24(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_25(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<NonTrivialMoveAssign> =
            Rc::new(RefCell::new(NonTrivialMoveAssign::new({ 5_() })));
        test_constexpr_assign_imp_26(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<NonTrivialMoveAssign> =
            Rc::new(RefCell::new(NonTrivialMoveAssign::new({ 5_() })));
        test_constexpr_assign_imp_26(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn non_constexpr_test_30() {
    ({ test_move_assignment_empty_empty_7() });
    ({ test_move_assignment_non_empty_empty_8() });
    ({ test_move_assignment_empty_non_empty_9() });
    ({ test_assignment_throw_20() });
}
pub fn cxx17_constexpr_test_31() -> () {
    ({ test_move_assignment_sfinae_6() });
    ({ test_move_assignment_noexcept_5() });
    ({ test_constexpr_move_assignment_trivial_27() });
    return true;
}
pub fn cxx20_constexpr_test_32() -> () {
    ({ test_move_assignment_same_index_10() });
    ({ test_move_assignment_different_index_17() });
    ({ test_constexpr_move_assignment_non_trivial_29() });
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
    ({ non_constexpr_test_30() });
    ({ cxx17_constexpr_test_31() });
    ({ cxx20_constexpr_test_32() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
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
pub trait MoveAssignImpl {
    fn move_assign(&self, o: Ptr<MoveAssign>) -> Ptr<MoveAssign>;
}
impl MoveAssignImpl for Ptr<MoveAssign> {
    fn move_assign(&self, o: Ptr<MoveAssign>) -> Ptr<MoveAssign> {
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!((*self), move_construct).write({ o.with(|__s| __s.move_construct.clone()) });
        field!((*self), move_assign).write({ o.with(|__s| __s.move_assign.clone()) });
        (*self)
            .with(|__s| __s.move_assign.clone())
            .with_mut(|__v| __v.prefix_inc());
        field!(o, value).write(-1_());
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
        field!(that, value).write(-1_());
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
    fn name(&self) -> ();
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> () {
        return unimplemented!();
    }
}
pub trait anon_11Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_11Impl for Ptr<anon_11> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (({ get_12(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_13Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_13Impl for Ptr<anon_13> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: (({ get_14(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_15Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_15Impl for Ptr<anon_15> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ({ get_16(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_18Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_18Impl for Ptr<anon_18> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: (({ get_14(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_19Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_19Impl for Ptr<anon_19> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: ({ get_16(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_3.with(|_| ());
    let _ = last_call_args_4.with(|_| ());
}
