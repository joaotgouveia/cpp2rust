extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
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
#[derive(
    Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor, Default,
)]
#[byte_size(48)]
pub struct CopyAssign {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub alive: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub copy_construct: Ptr<i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub copy_assign: Ptr<i32>,
    #[offset(32)]
    #[byte_size(8)]
    pub move_construct: Ptr<i32>,
    #[offset(40)]
    #[byte_size(8)]
    pub move_assign: Ptr<i32>,
}
impl CopyAssign {
    pub fn new(
        mut v: i32,
        mut alv: Ptr<i32>,
        mut cpy_ctr: Ptr<i32>,
        mut cpy_assi: Ptr<i32>,
        mut move_ctr: Ptr<i32>,
        mut move_assi: Ptr<i32>,
    ) -> Self {
        let __this: Value<CopyAssign> = Rc::new(RefCell::new(Self {
            value: v,
            alive: (alv).clone(),
            copy_construct: (cpy_ctr).clone(),
            copy_assign: (cpy_assi).clone(),
            move_construct: (move_ctr).clone(),
            move_assign: (move_assi).clone(),
        }));
        let this: Ptr<CopyAssign> = __this.as_pointer();
        this.with(|__s| __s.alive.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<CopyAssign>) -> Self {
        let __this: Value<CopyAssign> = Rc::new(RefCell::new(Self {
            value: o.with(|__s| __s.value),
            alive: o.with(|__s| __s.alive.clone()),
            copy_construct: o.with(|__s| __s.copy_construct.clone()),
            copy_assign: o.with(|__s| __s.copy_assign.clone()),
            move_construct: o.with(|__s| __s.move_construct.clone()),
            move_assign: o.with(|__s| __s.move_assign.clone()),
        }));
        let this: Ptr<CopyAssign> = __this.as_pointer();
        this.with(|__s| __s.alive.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        this.with(|__s| __s.copy_construct.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<CopyAssign>) -> Self {
        let __this: Value<CopyAssign> = Rc::new(RefCell::new(Self {
            value: o.with(|__s| __s.value),
            alive: o.with(|__s| __s.alive.clone()),
            copy_construct: o.with(|__s| __s.copy_construct.clone()),
            copy_assign: o.with(|__s| __s.copy_assign.clone()),
            move_construct: o.with(|__s| __s.move_construct.clone()),
            move_assign: o.with(|__s| __s.move_assign.clone()),
        }));
        let this: Ptr<CopyAssign> = __this.as_pointer();
        field!(o, value).write(-1_i32);
        this.with(|__s| __s.alive.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        this.with(|__s| __s.move_construct.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for CopyAssign {
    fn clone(&self) -> Self {
        let __src: Value<CopyAssign> = Rc::new(RefCell::new(CopyAssign {
            value: self.value.clone(),
            alive: self.alive.clone(),
            copy_construct: self.copy_construct.clone(),
            copy_assign: self.copy_assign.clone(),
            move_construct: self.move_construct.clone(),
            move_assign: self.move_assign.clone(),
        }));
        CopyAssign::copy_from(__src.as_pointer())
    }
}
impl CppMove for CopyAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyMaybeThrows {}
impl CopyMaybeThrows {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyDoesThrow {}
impl CopyDoesThrow {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NTCopyAssign {
    #[offset(0)]
    pub value: i32,
}
impl NTCopyAssign {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppMove for NTCopyAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_assignable<NTCopyAssign>::value"
);
const _: () = assert!(true, "std::is_copy_assignable<NTCopyAssign>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct TCopyAssign {
    #[offset(0)]
    pub value: i32,
}
impl TCopyAssign {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppMove for TCopyAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable<TCopyAssign>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct TCopyAssignNTMoveAssign {
    #[offset(0)]
    pub value: i32,
}
impl TCopyAssignNTMoveAssign {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppMove for TCopyAssignNTMoveAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable_v<TCopyAssignNTMoveAssign>"
);
pub fn test_copy_assignment_not_noexcept_0() {
    {
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<V>::value");;
    }
}
pub fn test_copy_assignment_sfinae_1() {
    {
        const _: () = assert!(true, "std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_copy_assignable<V>::value");;
        const _: () = assert!(true, "std::is_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    }
}
pub fn test_copy_assignment_empty_empty_2() {}
pub fn test_copy_assignment_non_empty_empty_3() {}
pub fn test_copy_assignment_empty_non_empty_4() {}
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
pub fn test_copy_assignment_same_index_5() {
    {
        let v1: Value<std_variant_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int_> = Rc::new(RefCell::new(42));
        let vref: Ptr<std_variant_int_> = ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 0_usize) }));
        assert!(((({ get_6(v1.as_pointer(),) }).read()) == 42));
    }
    {
        let v1: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43_i64));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        let vref: Ptr<std_variant_int__long__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!(((({ get_7(v1.as_pointer(),) }).read()) == 42_i64));
    }
    {
        let alive: Value<i32> = Rc::new(RefCell::new(0));
        let copy_construct: Value<i32> = Rc::new(RefCell::new(0));
        let copy_assign: Value<i32> = Rc::new(RefCell::new(0));
        let move_construct: Value<i32> = Rc::new(RefCell::new(0));
        let move_assign: Value<i32> = Rc::new(RefCell::new(0));
        let v1: Value<std_variant_int__CopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(43));
            let __tmp_1: Value<Ptr<i32>> = Rc::new(RefCell::new((alive.as_pointer())));
            let __tmp_2: Value<Ptr<i32>> = Rc::new(RefCell::new((copy_construct.as_pointer())));
            let __tmp_3: Value<Ptr<i32>> = Rc::new(RefCell::new((copy_assign.as_pointer())));
            let __tmp_4: Value<Ptr<i32>> = Rc::new(RefCell::new((move_construct.as_pointer())));
            let __tmp_5: Value<Ptr<i32>> = Rc::new(RefCell::new((move_assign.as_pointer())));
            std_variant_int__CopyAssign__unsigned_int_::new_1(
                { (*in_place_type_8.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
                { __tmp_1.as_pointer() },
                { __tmp_2.as_pointer() },
                { __tmp_3.as_pointer() },
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__CopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_6: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_7: Value<Ptr<i32>> = Rc::new(RefCell::new((alive.as_pointer())));
            let __tmp_8: Value<Ptr<i32>> = Rc::new(RefCell::new((copy_construct.as_pointer())));
            let __tmp_9: Value<Ptr<i32>> = Rc::new(RefCell::new((copy_assign.as_pointer())));
            let __tmp_10: Value<Ptr<i32>> = Rc::new(RefCell::new((move_construct.as_pointer())));
            let __tmp_11: Value<Ptr<i32>> = Rc::new(RefCell::new((move_assign.as_pointer())));
            std_variant_int__CopyAssign__unsigned_int_::new_1(
                { (*in_place_type_8.with(Value::clone).borrow()).clone() },
                { __tmp_6.as_pointer() },
                { __tmp_7.as_pointer() },
                { __tmp_8.as_pointer() },
                { __tmp_9.as_pointer() },
                { __tmp_10.as_pointer() },
                { __tmp_11.as_pointer() },
            )
        }));
        let vref: Ptr<std_variant_int__CopyAssign__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!((({ get_9(v1.as_pointer(),) }).with(|__s| __s.value) == 42));
        assert!(({ ((*copy_construct.borrow()) == 0) }));
        assert!(({ ((*move_construct.borrow()) == 0) }));
        assert!(({ ((*copy_assign.borrow()) == 1) }));
    }
    {
        let test: Value<anon_10> = Rc::new(RefCell::new(<anon_10>::default()));
        let mut result: Result_int_ = ({ anon_10Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_11> = Rc::new(RefCell::new(<anon_11>::default()));
        let mut result: Result_long_ = ({ anon_11Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_12> = Rc::new(RefCell::new(<anon_12>::default()));
        let mut result: Result_int_ = ({ anon_12Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
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
pub struct anon_10 {}
impl Callable0<Result_int_> for anon_10 {
    fn call(&self) -> Result_int_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_10Impl::operator_call(__it))
    }
}
impl CppMove for anon_10 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_11 {}
impl Callable0<Result_long_> for anon_11 {
    fn call(&self) -> Result_long_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_11Impl::operator_call(__it))
    }
}
impl CppMove for anon_11 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_12 {}
impl Callable0<Result_int_> for anon_12 {
    fn call(&self) -> Result_int_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_12Impl::operator_call(__it))
    }
}
impl CppMove for anon_12 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
pub fn test_copy_assignment_different_index_18() {
    {
        let v1: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        let vref: Ptr<std_variant_int__long__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!(((({ get_7(v1.as_pointer(),) }).read()) == 42_i64));
    }
    {
        let alive: Value<i32> = Rc::new(RefCell::new(0));
        let copy_construct: Value<i32> = Rc::new(RefCell::new(0));
        let copy_assign: Value<i32> = Rc::new(RefCell::new(0));
        let move_construct: Value<i32> = Rc::new(RefCell::new(0));
        let move_assign: Value<i32> = Rc::new(RefCell::new(0));
        let v1: Value<std_variant_int__CopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_16: Value<u32> = Rc::new(RefCell::new(43_u32));
            std_variant_int__CopyAssign__unsigned_int_::new_2(
                { (*in_place_type_19.with(Value::clone).borrow()).clone() },
                { __tmp_16.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__CopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_17: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_18: Value<Ptr<i32>> = Rc::new(RefCell::new((alive.as_pointer())));
            let __tmp_19: Value<Ptr<i32>> = Rc::new(RefCell::new((copy_construct.as_pointer())));
            let __tmp_20: Value<Ptr<i32>> = Rc::new(RefCell::new((copy_assign.as_pointer())));
            let __tmp_21: Value<Ptr<i32>> = Rc::new(RefCell::new((move_construct.as_pointer())));
            let __tmp_22: Value<Ptr<i32>> = Rc::new(RefCell::new((move_assign.as_pointer())));
            std_variant_int__CopyAssign__unsigned_int_::new_1(
                { (*in_place_type_8.with(Value::clone).borrow()).clone() },
                { __tmp_17.as_pointer() },
                { __tmp_18.as_pointer() },
                { __tmp_19.as_pointer() },
                { __tmp_20.as_pointer() },
                { __tmp_21.as_pointer() },
                { __tmp_22.as_pointer() },
            )
        }));
        assert!(({ ((*copy_construct.borrow()) == 0) }));
        assert!(({ ((*move_construct.borrow()) == 0) }));
        assert!(({ ((*alive.borrow()) == 1) }));
        let vref: Ptr<std_variant_int__CopyAssign__unsigned_int_> =
            ((*v1.borrow_mut()) = (*v2.borrow()).clone());
        assert!(({ (vref).clone() } == { (v1.as_pointer()) }));
        assert!(({ (({ (*v1.borrow()).index() }) == 1_usize) }));
        assert!((({ get_9(v1.as_pointer(),) }).with(|__s| __s.value) == 42));
        assert!(({ ((*alive.borrow()) == 2) }));
        assert!(({ ((*copy_construct.borrow()) == 1) }));
        assert!(({ ((*move_construct.borrow()) == 1) }));
        assert!(({ ((*copy_assign.borrow()) == 0) }));
    }
    {
        let test: Value<anon_20> = Rc::new(RefCell::new(<anon_20>::default()));
        let mut result: Result_long_ = ({ anon_20Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_21> = Rc::new(RefCell::new(<anon_21>::default()));
        let mut result: Result_int_ = ({ anon_21Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_20 {}
impl Callable0<Result_long_> for anon_20 {
    fn call(&self) -> Result_long_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_20Impl::operator_call(__it))
    }
}
impl CppMove for anon_20 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_21 {}
impl Callable0<Result_int_> for anon_21 {
    fn call(&self) -> Result_int_ {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| anon_21Impl::operator_call(__it))
    }
}
impl CppMove for anon_21 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_assignment_throw_22() {}
pub fn test_constexpr_copy_assignment_trivial_23() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(42_i64));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_24(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(Default::default()));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_24(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(42_i64));
        let _new_value: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_25(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__int_> = Rc::new(RefCell::new(42_i64));
        let _new_value: Value<i32> = Rc::new(RefCell::new(101));
        test_constexpr_assign_imp_26(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn operator_eq_27(x: Ptr<NonTrivialCopyAssign>, y: Ptr<NonTrivialCopyAssign>) -> bool {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NonTrivialCopyAssign {
    #[offset(0)]
    pub i: i32,
}
impl NonTrivialCopyAssign {
    pub fn new(mut ii: i32) -> Self {
        Self { i: ii }
    }
    pub fn copy_from(other: Ptr<NonTrivialCopyAssign>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
}
impl CppEq for NonTrivialCopyAssign {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_27(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for NonTrivialCopyAssign {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivialCopyAssign> =
            Rc::new(RefCell::new(NonTrivialCopyAssign { i: self.i.clone() }));
        NonTrivialCopyAssign::copy_from(__src.as_pointer())
    }
}
impl CppMove for NonTrivialCopyAssign {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for NonTrivialCopyAssign {
    fn default() -> Self {
        { NonTrivialCopyAssign { i: 0 } }
    }
}
pub fn test_constexpr_copy_assignment_non_trivial_28() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_copy_assignable<V>::value");;
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialCopyAssign_> =
            Rc::new(RefCell::new(42_i64));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_29(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialCopyAssign_> =
            Rc::new(RefCell::new(Default::default()));
        let _new_value: Value<i64> = Rc::new(RefCell::new(101_i64));
        test_constexpr_assign_imp_29(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialCopyAssign_> =
            Rc::new(RefCell::new(42_i64));
        let _new_value: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_30(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialCopyAssign_> =
            Rc::new(RefCell::new(42_i64));
        let _new_value: Value<NonTrivialCopyAssign> =
            Rc::new(RefCell::new(NonTrivialCopyAssign::new({ 5 })));
        test_constexpr_assign_imp_31(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<std_variant_long__void_ptr__NonTrivialCopyAssign_> =
            Rc::new(RefCell::new(NonTrivialCopyAssign::new({ 3 })));
        let _new_value: Value<NonTrivialCopyAssign> =
            Rc::new(RefCell::new(NonTrivialCopyAssign::new({ 5 })));
        test_constexpr_assign_imp_31(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn non_constexpr_test_32() {
    ({ test_copy_assignment_empty_empty_2() });
    ({ test_copy_assignment_non_empty_empty_3() });
    ({ test_copy_assignment_empty_non_empty_4() });
    ({ test_assignment_throw_22() });
}
pub fn cxx17_constexpr_test_33() -> bool {
    ({ test_copy_assignment_sfinae_1() });
    ({ test_copy_assignment_not_noexcept_0() });
    ({ test_constexpr_copy_assignment_trivial_23() });
    return true;
}
pub fn cxx20_constexpr_test_34() -> bool {
    ({ test_copy_assignment_same_index_5() });
    ({ test_copy_assignment_different_index_18() });
    ({ test_constexpr_copy_assignment_non_trivial_28() });
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
    ({ non_constexpr_test_32() });
    ({ cxx17_constexpr_test_33() });
    ({ cxx20_constexpr_test_34() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0;
}
pub trait CopyAssignImpl {
    fn copy_assign(&self, o: Ptr<CopyAssign>) -> Ptr<CopyAssign>;
    fn move_assign(&self, o: Ptr<CopyAssign>) -> Ptr<CopyAssign>;
    fn destructor(&self);
}
impl CopyAssignImpl for Ptr<CopyAssign> {
    fn copy_assign(&self, o: Ptr<CopyAssign>) -> Ptr<CopyAssign> {
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!((*self), alive).write({ o.with(|__s| __s.alive.clone()) });
        field!((*self), copy_construct).write({ o.with(|__s| __s.copy_construct.clone()) });
        field!((*self), copy_assign).write({ o.with(|__s| __s.copy_assign.clone()) });
        field!((*self), move_construct).write({ o.with(|__s| __s.move_construct.clone()) });
        field!((*self), move_assign).write({ o.with(|__s| __s.move_assign.clone()) });
        (*self).with(|__s| __s.copy_assign.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<CopyAssign>) -> Ptr<CopyAssign> {
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!((*self), alive).write({ o.with(|__s| __s.alive.clone()) });
        field!((*self), copy_construct).write({ o.with(|__s| __s.copy_construct.clone()) });
        field!((*self), copy_assign).write({ o.with(|__s| __s.copy_assign.clone()) });
        field!((*self), move_construct).write({ o.with(|__s| __s.move_construct.clone()) });
        field!((*self), move_assign).write({ o.with(|__s| __s.move_assign.clone()) });
        field!(o, value).write(-1_i32);
        (*self).with(|__s| __s.move_assign.clone()).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        (*self).with(|__s| __s.alive.clone()).with_mut(|__v| {
            __v.prefix_dec();
        });
    }
}
pub trait CopyDoesThrowImpl {
    fn copy_assign(&self, _a0: Ptr<CopyDoesThrow>) -> Ptr<CopyDoesThrow> {
        unimplemented!()
    }
}
impl CopyDoesThrowImpl for Ptr<CopyDoesThrow> {}
pub trait CopyMaybeThrowsImpl {
    fn copy_assign(&self, _a0: Ptr<CopyMaybeThrows>) -> Ptr<CopyMaybeThrows> {
        unimplemented!()
    }
}
impl CopyMaybeThrowsImpl for Ptr<CopyMaybeThrows> {}
pub trait NTCopyAssignImpl {
    fn copy_assign(&self, that: Ptr<NTCopyAssign>) -> Ptr<NTCopyAssign>;
}
impl NTCopyAssignImpl for Ptr<NTCopyAssign> {
    fn copy_assign(&self, that: Ptr<NTCopyAssign>) -> Ptr<NTCopyAssign> {
        field!((*self), value).write({ that.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub trait NonTrivialCopyAssignImpl {
    fn copy_assign(&self, o: Ptr<NonTrivialCopyAssign>) -> Ptr<NonTrivialCopyAssign>;
}
impl NonTrivialCopyAssignImpl for Ptr<NonTrivialCopyAssign> {
    fn copy_assign(&self, o: Ptr<NonTrivialCopyAssign>) -> Ptr<NonTrivialCopyAssign> {
        field!((*self), i).write({ o.with(|__s| __s.i) });
        return (*self).clone();
    }
}
pub trait TCopyAssignNTMoveAssignImpl {
    fn move_assign(&self, that: Ptr<TCopyAssignNTMoveAssign>) -> Ptr<TCopyAssignNTMoveAssign>;
}
impl TCopyAssignNTMoveAssignImpl for Ptr<TCopyAssignNTMoveAssign> {
    fn move_assign(&self, that: Ptr<TCopyAssignNTMoveAssign>) -> Ptr<TCopyAssignNTMoveAssign> {
        field!((*self), value).write({ that.with(|__s| __s.value) });
        field!(that, value).write(-1_i32);
        return (*self).clone();
    }
}
pub trait anon_10Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_10Impl for Ptr<anon_10> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<std_variant_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int_> = Rc::new(RefCell::new(42));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_int_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_6(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_11Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_11Impl for Ptr<anon_11> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43_i64));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_long_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_7(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_12Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_12Impl for Ptr<anon_12> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<std_variant_int__TCopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_12: Value<i32> = Rc::new(RefCell::new(43));
            std_variant_int__TCopyAssign__unsigned_int_::new_1(
                { (*in_place_type_13.with(Value::clone).borrow()).clone() },
                { __tmp_12.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__TCopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_13: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__TCopyAssign__unsigned_int_::new_1(
                { (*in_place_type_13.with(Value::clone).borrow()).clone() },
                { __tmp_13.as_pointer() },
            )
        }));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_int_ {
            index: ({ (*v.borrow()).index() }),
            value: ({ get_14(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_15Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_15Impl for Ptr<anon_15> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<std_variant_int__TCopyAssignNTMoveAssign__unsigned_int_> =
            Rc::new(RefCell::new({
                let __tmp_14: Value<i32> = Rc::new(RefCell::new(43));
                std_variant_int__TCopyAssignNTMoveAssign__unsigned_int_::new_1(
                    { (*in_place_type_16.with(Value::clone).borrow()).clone() },
                    { __tmp_14.as_pointer() },
                )
            }));
        let v2: Value<std_variant_int__TCopyAssignNTMoveAssign__unsigned_int_> =
            Rc::new(RefCell::new({
                let __tmp_15: Value<i32> = Rc::new(RefCell::new(42));
                std_variant_int__TCopyAssignNTMoveAssign__unsigned_int_::new_1(
                    { (*in_place_type_16.with(Value::clone).borrow()).clone() },
                    { __tmp_15.as_pointer() },
                )
            }));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_int_ {
            index: ({ (*v.borrow()).index() }),
            value: ({ get_17(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_20Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_20Impl for Ptr<anon_20> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(43));
        let v2: Value<std_variant_int__long__unsigned_int_> = Rc::new(RefCell::new(42_i64));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_long_ {
            index: ({ (*v.borrow()).index() }),
            value: (({ get_7(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_21Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_21Impl for Ptr<anon_21> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<std_variant_int__TCopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_23: Value<u32> = Rc::new(RefCell::new(43_u32));
            std_variant_int__TCopyAssign__unsigned_int_::new_2(
                { (*in_place_type_19.with(Value::clone).borrow()).clone() },
                { __tmp_23.as_pointer() },
            )
        }));
        let v2: Value<std_variant_int__TCopyAssign__unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_24: Value<i32> = Rc::new(RefCell::new(42));
            std_variant_int__TCopyAssign__unsigned_int_::new_1(
                { (*in_place_type_13.with(Value::clone).borrow()).clone() },
                { __tmp_24.as_pointer() },
            )
        }));
        (*v.borrow_mut()) = (*v2.borrow()).clone();
        return Result_int_ {
            index: ({ (*v.borrow()).index() }),
            value: ({ get_14(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub fn __cpp2rust_init_globals() {}
