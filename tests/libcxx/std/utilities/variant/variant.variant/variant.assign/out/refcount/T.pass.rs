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
pub struct MetaHelpers_Dummy {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(1)]
pub struct MetaHelpers_ThrowsCtorT {}
impl MetaHelpers_ThrowsCtorT {
    pub fn new(mut _a0: i32) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(1)]
pub struct MetaHelpers_ThrowsAssignT {}
impl MetaHelpers_ThrowsAssignT {
    pub fn new(mut _a0: i32) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(1)]
pub struct MetaHelpers_NoThrowT {}
impl MetaHelpers_NoThrowT {
    pub fn new(mut _a0: i32) -> Self {
        Self {}
    }
}
pub fn test_T_assignment_noexcept_5() {
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
pub fn test_T_assignment_sfinae_6() {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Y_9 {}
pub fn test_T_assignment_basic_10() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 0_usize));
        assert!(((unimplemented!().read()) == 42));
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 0_usize));
        assert!(((unimplemented!().read()) == 42));
        unimplemented!();
        assert!((unimplemented!() == 1_usize));
        assert!(((unimplemented!().read()) == 43_()));
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 1_usize));
        assert!(((unimplemented!().read()) == 42_()));
        unimplemented!();
        assert!((unimplemented!() == 0_usize));
        assert!(((unimplemented!().read()) == 43_()));
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 0_usize));
        assert!(
            Ptr::<i8>::from_string_literal(b"bar").with_c_str(|__s| (unimplemented!()).clone()
                [..(unimplemented!()).clone().len().saturating_sub(1)]
                == *__s)
        );
    }
}
pub fn test_T_assignment_basic_no_constexpr_11() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((unimplemented!() == 1_usize));
    assert!((unimplemented!()).clone().is_none());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct TraceStat {
    #[offset(0)]
    pub construct: i32,
    #[offset(4)]
    pub copy_construct: i32,
    #[offset(8)]
    pub copy_assign: i32,
    #[offset(12)]
    pub move_construct: i32,
    #[offset(16)]
    pub move_assign: i32,
    #[offset(20)]
    pub T_copy_assign: i32,
    #[offset(24)]
    pub T_move_assign: i32,
    #[offset(28)]
    pub destroy: i32,
}
impl Default for TraceStat {
    fn default() -> Self {
        {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Trace_false__false__T {}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, MoveAssign, Destructor, Default)]
#[cpp_from(Ptr< TraceStat  >  , new_1, value)]
#[cpp_from(Trace_false__false__T, new_2, value)]
#[byte_size(8)]
pub struct Trace_false__false_ {
    #[offset(0)]
    #[byte_size(8)]
    pub stat: Ptr<TraceStat>,
}
impl Trace_false__false_ {
    pub fn new_1(mut s: Ptr<TraceStat>) -> Self {
        let __this: Trace_false__false_ = Self { stat: (s).clone() };
        field!(s, construct).with_mut(|__v| __v.prefix_inc());
        __this
    }
    pub fn new_2(mut _a0: Trace_false__false__T) -> Self {
        Self {
            stat: Ptr::<TraceStat>::null(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Trace_false__true__T {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Destructor, Default)]
#[cpp_from(Ptr< TraceStat  >  , new_1, value)]
#[byte_size(8)]
pub struct Trace_false__true_ {
    #[offset(0)]
    #[byte_size(8)]
    pub stat: Ptr<TraceStat>,
}
impl Trace_false__true_ {
    pub fn new_1(mut s: Ptr<TraceStat>) -> Self {
        let __this: Trace_false__true_ = Self { stat: (s).clone() };
        field!(s, construct).with_mut(|__v| __v.prefix_inc());
        __this
    }
    pub fn move_from(o: Ptr<Trace_false__true_>) -> Self {
        let __this: Value<Trace_false__true_> = Rc::new(RefCell::new(Self {
            stat: o.with(|__s| __s.stat.clone()),
        }));
        let this: Ptr<Trace_false__true_> = __this.as_pointer();
        field!(this.with(|__s| __s.stat.clone()), move_construct).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Trace_true__false__T {}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, MoveAssign, Destructor, Default)]
#[cpp_from(Ptr< TraceStat  >  , new_1, value)]
#[byte_size(8)]
pub struct Trace_true__false_ {
    #[offset(0)]
    #[byte_size(8)]
    pub stat: Ptr<TraceStat>,
}
impl Trace_true__false_ {
    pub fn new_1(mut s: Ptr<TraceStat>) -> Self {
        let __this: Trace_true__false_ = Self { stat: (s).clone() };
        field!(s, construct).with_mut(|__v| __v.prefix_inc());
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Trace_true__true__T {}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, MoveAssign, Destructor, Default)]
#[cpp_from(Ptr< TraceStat  >  , new_1, value)]
#[byte_size(8)]
pub struct Trace_true__true_ {
    #[offset(0)]
    #[byte_size(8)]
    pub stat: Ptr<TraceStat>,
}
impl Trace_true__true_ {
    pub fn new_1(mut s: Ptr<TraceStat>) -> Self {
        let __this: Trace_true__true_ = Self { stat: (s).clone() };
        field!(s, construct).with_mut(|__v| __v.prefix_inc());
        __this
    }
}
pub fn test_T_assignment_performs_construction_12() {
    {
        let stat: Value<TraceStat> = Rc::new(RefCell::new(<TraceStat>::default()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ (*stat.borrow()).construct } == 1));
        assert!(({ (*stat.borrow()).copy_construct } == 0));
        assert!(({ (*stat.borrow()).move_construct } == 0));
        assert!(({ (*stat.borrow()).copy_assign } == 0));
        assert!(({ (*stat.borrow()).move_assign } == 0));
        assert!(({ (*stat.borrow()).destroy } == 0));
    }
    {
        let stat: Value<TraceStat> = Rc::new(RefCell::new(<TraceStat>::default()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ (*stat.borrow()).construct } == 1));
        assert!(({ (*stat.borrow()).copy_construct } == 0));
        assert!(({ (*stat.borrow()).move_construct } == 1));
        assert!(({ (*stat.borrow()).copy_assign } == 0));
        assert!(({ (*stat.borrow()).move_assign } == 0));
        assert!(({ (*stat.borrow()).destroy } == 1));
    }
    {
        let stat: Value<TraceStat> = Rc::new(RefCell::new(<TraceStat>::default()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ (*stat.borrow()).construct } == 1));
        assert!(({ (*stat.borrow()).copy_construct } == 0));
        assert!(({ (*stat.borrow()).move_construct } == 0));
        assert!(({ (*stat.borrow()).copy_assign } == 0));
        assert!(({ (*stat.borrow()).move_assign } == 0));
        assert!(({ (*stat.borrow()).destroy } == 0));
    }
    {
        let stat: Value<TraceStat> = Rc::new(RefCell::new(<TraceStat>::default()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ (*stat.borrow()).construct } == 1));
        assert!(({ (*stat.borrow()).copy_construct } == 0));
        assert!(({ (*stat.borrow()).move_construct } == 0));
        assert!(({ (*stat.borrow()).copy_assign } == 0));
        assert!(({ (*stat.borrow()).move_assign } == 0));
        assert!(({ (*stat.borrow()).destroy } == 0));
    }
}
pub fn test_T_assignment_performs_assignment_13() {
    {
        let stat: Value<TraceStat> = Rc::new(RefCell::new(<TraceStat>::default()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ (*stat.borrow()).construct } == 1));
        assert!(({ (*stat.borrow()).copy_construct } == 0));
        assert!(({ (*stat.borrow()).move_construct } == 0));
        assert!(({ (*stat.borrow()).copy_assign } == 0));
        assert!(({ (*stat.borrow()).move_assign } == 0));
        assert!(({ (*stat.borrow()).T_copy_assign } == 0));
        assert!(({ (*stat.borrow()).T_move_assign } == 1));
        assert!(({ (*stat.borrow()).destroy } == 0));
    }
    {
        let stat: Value<TraceStat> = Rc::new(RefCell::new(<TraceStat>::default()));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut t: Trace_false__false__T = <Trace_false__false__T>::default();
        unimplemented!();
        assert!(({ (*stat.borrow()).construct } == 1));
        assert!(({ (*stat.borrow()).copy_construct } == 0));
        assert!(({ (*stat.borrow()).move_construct } == 0));
        assert!(({ (*stat.borrow()).copy_assign } == 0));
        assert!(({ (*stat.borrow()).move_assign } == 0));
        assert!(({ (*stat.borrow()).T_copy_assign } == 1));
        assert!(({ (*stat.borrow()).T_move_assign } == 0));
        assert!(({ (*stat.borrow()).destroy } == 0));
    }
}
pub fn test_T_assignment_performs_construction_throw_14() {}
pub fn test_T_assignment_performs_assignment_throw_15() {}
pub fn test_T_assignment_vector_bool_16() {
    let vec_: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::from([true])));
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((unimplemented!() == 0_usize));
    assert!((((unimplemented!().read()) as i32) == (true as i32)));
}
pub fn non_constexpr_test_17() {
    ({ test_T_assignment_basic_no_constexpr_11() });
    ({ test_T_assignment_performs_construction_throw_14() });
    ({ test_T_assignment_performs_assignment_throw_15() });
}
pub fn test_18() -> bool {
    ({ test_T_assignment_basic_10() });
    ({ test_T_assignment_performs_construction_12() });
    ({ test_T_assignment_performs_assignment_13() });
    ({ test_T_assignment_noexcept_5() });
    ({ test_T_assignment_sfinae_6() });
    ({ test_T_assignment_vector_bool_16() });
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
    ({ test_18() });
    ({ non_constexpr_test_17() });
    const _: () = assert!(true, "test()");;
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
pub trait MetaHelpers_NoThrowTImpl {
    fn operator_assign_2(&self, _a0: i32) -> Ptr<MetaHelpers_NoThrowT>;
}
impl MetaHelpers_NoThrowTImpl for Ptr<MetaHelpers_NoThrowT> {
    fn operator_assign_2(&self, mut _a0: i32) -> Ptr<MetaHelpers_NoThrowT> {
        return (*self).clone();
    }
}
pub trait MetaHelpers_ThrowsAssignTImpl {
    fn operator_assign_2(&self, _a0: i32) -> Ptr<MetaHelpers_ThrowsAssignT>;
}
impl MetaHelpers_ThrowsAssignTImpl for Ptr<MetaHelpers_ThrowsAssignT> {
    fn operator_assign_2(&self, mut _a0: i32) -> Ptr<MetaHelpers_ThrowsAssignT> {
        return (*self).clone();
    }
}
pub trait MetaHelpers_ThrowsCtorTImpl {
    fn operator_assign_2(&self, _a0: i32) -> Ptr<MetaHelpers_ThrowsCtorT>;
}
impl MetaHelpers_ThrowsCtorTImpl for Ptr<MetaHelpers_ThrowsCtorT> {
    fn operator_assign_2(&self, mut _a0: i32) -> Ptr<MetaHelpers_ThrowsCtorT> {
        return (*self).clone();
    }
}
pub trait Trace_false__false_Impl {
    fn copy_assign(&self, _a0: Ptr<Trace_false__false_>) -> Ptr<Trace_false__false_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<Trace_false__false_>) -> Ptr<Trace_false__false_>;
    fn operator_assign_7(&self, _a0: Ptr<Trace_false__false__T>) -> Ptr<Trace_false__false_>;
    fn operator_assign_8(&self, _a0: Ptr<Trace_false__false__T>) -> Ptr<Trace_false__false_>;
    fn destructor(&self);
}
impl Trace_false__false_Impl for Ptr<Trace_false__false_> {
    fn move_assign(&self, _a0: Ptr<Trace_false__false_>) -> Ptr<Trace_false__false_> {
        field!((*self).with(|__s| __s.stat.clone()), move_assign).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_assign_7(&self, _a0: Ptr<Trace_false__false__T>) -> Ptr<Trace_false__false_> {
        field!((*self).with(|__s| __s.stat.clone()), T_copy_assign)
            .with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_assign_8(&self, _a0: Ptr<Trace_false__false__T>) -> Ptr<Trace_false__false_> {
        field!((*self).with(|__s| __s.stat.clone()), T_move_assign)
            .with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn destructor(&self) {
        field!((*self).with(|__s| __s.stat.clone()), destroy).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait Trace_false__true_Impl {
    fn copy_assign(&self, _a0: Ptr<Trace_false__true_>) -> Ptr<Trace_false__true_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<Trace_false__true_>) -> Ptr<Trace_false__true_>;
    fn operator_assign_7(&self, _a0: Ptr<Trace_false__true__T>) -> Ptr<Trace_false__true_> {
        unimplemented!()
    }
    fn operator_assign_8(&self, _a0: Ptr<Trace_false__true__T>) -> Ptr<Trace_false__true_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl Trace_false__true_Impl for Ptr<Trace_false__true_> {
    fn move_assign(&self, _a0: Ptr<Trace_false__true_>) -> Ptr<Trace_false__true_> {
        field!((*self).with(|__s| __s.stat.clone()), move_assign).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn destructor(&self) {
        field!((*self).with(|__s| __s.stat.clone()), destroy).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait Trace_true__false_Impl {
    fn copy_assign(&self, _a0: Ptr<Trace_true__false_>) -> Ptr<Trace_true__false_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<Trace_true__false_>) -> Ptr<Trace_true__false_>;
    fn operator_assign_7(&self, _a0: Ptr<Trace_true__false__T>) -> Ptr<Trace_true__false_> {
        unimplemented!()
    }
    fn operator_assign_8(&self, _a0: Ptr<Trace_true__false__T>) -> Ptr<Trace_true__false_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl Trace_true__false_Impl for Ptr<Trace_true__false_> {
    fn move_assign(&self, _a0: Ptr<Trace_true__false_>) -> Ptr<Trace_true__false_> {
        field!((*self).with(|__s| __s.stat.clone()), move_assign).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn destructor(&self) {
        field!((*self).with(|__s| __s.stat.clone()), destroy).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait Trace_true__true_Impl {
    fn copy_assign(&self, _a0: Ptr<Trace_true__true_>) -> Ptr<Trace_true__true_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<Trace_true__true_>) -> Ptr<Trace_true__true_>;
    fn operator_assign_7(&self, _a0: Ptr<Trace_true__true__T>) -> Ptr<Trace_true__true_> {
        unimplemented!()
    }
    fn operator_assign_8(&self, _a0: Ptr<Trace_true__true__T>) -> Ptr<Trace_true__true_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl Trace_true__true_Impl for Ptr<Trace_true__true_> {
    fn move_assign(&self, _a0: Ptr<Trace_true__true_>) -> Ptr<Trace_true__true_> {
        field!((*self).with(|__s| __s.stat.clone()), move_assign).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn destructor(&self) {
        field!((*self).with(|__s| __s.stat.clone()), destroy).with_mut(|__v| __v.prefix_inc());
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
pub trait X_7Impl {
    fn to_AnyPtr(&self) -> AnyPtr {
        unimplemented!()
    }
}
impl X_7Impl for Ptr<X_7> {}
pub trait Y_9Impl {
    fn to_X_8(&self) -> X_8 {
        unimplemented!()
    }
}
impl Y_9Impl for Ptr<Y_9> {}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_3.with(|_| ());
    let _ = last_call_args_4.with(|_| ());
}
