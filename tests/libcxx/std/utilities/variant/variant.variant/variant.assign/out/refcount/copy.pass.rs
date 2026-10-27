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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct MoveOnly {}
impl MoveOnly {
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {}
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
#[derive(
    Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor, Default,
)]
#[byte_size(48)]
pub struct CopyAssign {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub alive: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub copy_construct: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub copy_assign: Ptr<()>,
    #[offset(32)]
    #[byte_size(8)]
    pub move_construct: Ptr<()>,
    #[offset(40)]
    #[byte_size(8)]
    pub move_assign: Ptr<()>,
}
impl CopyAssign {
    pub fn new(
        mut v: (),
        mut alv: Ptr<()>,
        mut cpy_ctr: Ptr<()>,
        mut cpy_assi: Ptr<()>,
        mut move_ctr: Ptr<()>,
        mut move_assi: Ptr<()>,
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
        this.with(|__s| __s.alive.clone())
            .with_mut(|__v| __v.prefix_inc());
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
        this.with(|__s| __s.alive.clone())
            .with_mut(|__v| __v.prefix_inc());
        this.with(|__s| __s.copy_construct.clone())
            .with_mut(|__v| __v.prefix_inc());
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
        field!(o, value).write(-1_());
        this.with(|__s| __s.alive.clone())
            .with_mut(|__v| __v.prefix_inc());
        this.with(|__s| __s.move_construct.clone())
            .with_mut(|__v| __v.prefix_inc());
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
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyMaybeThrows {}
impl CopyMaybeThrows {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyDoesThrow {}
impl CopyDoesThrow {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct NTCopyAssign {
    #[offset(0)]
    pub value: (),
}
impl NTCopyAssign {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_assignable<NTCopyAssign>::value"
);
const _: () = assert!(true, "std::is_copy_assignable<NTCopyAssign>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TCopyAssign {
    #[offset(0)]
    pub value: (),
}
impl TCopyAssign {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_assignable<TCopyAssign>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveAssign, Default)]
#[byte_size(4)]
pub struct TCopyAssignNTMoveAssign {
    #[offset(0)]
    pub value: (),
}
impl TCopyAssignNTMoveAssign {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
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
pub fn test_copy_assignment_same_index_5() {
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
        let alive: Value<()> = Rc::new(RefCell::new(0_()));
        let copy_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let copy_assign: Value<()> = Rc::new(RefCell::new(0_()));
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
        unimplemented!();
    }
    {
        let test: Value<anon_6> = Rc::new(RefCell::new(<anon_6>::default()));
        let mut result: Result_int_ = ({ anon_6Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_8> = Rc::new(RefCell::new(<anon_8>::default()));
        let mut result: Result_long_ = ({ anon_8Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_10> = Rc::new(RefCell::new(<anon_10>::default()));
        let mut result: Result_int_ = ({ anon_10Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_12> = Rc::new(RefCell::new(<anon_12>::default()));
        let mut result: Result_int_ = ({ anon_12Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_6 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_10 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_12 {}
pub fn test_copy_assignment_different_index_14() {
    {
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let alive: Value<()> = Rc::new(RefCell::new(0_()));
        let copy_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let copy_assign: Value<()> = Rc::new(RefCell::new(0_()));
        let move_construct: Value<()> = Rc::new(RefCell::new(0_()));
        let move_assign: Value<()> = Rc::new(RefCell::new(0_()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let vref: Ptr<()> = (unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let test: Value<anon_15> = Rc::new(RefCell::new(<anon_15>::default()));
        let mut result: Result_long_ = ({ anon_15Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42l");;
    }
    {
        let test: Value<anon_16> = Rc::new(RefCell::new(<anon_16>::default()));
        let mut result: Result_int_ = ({ anon_16Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_15 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_16 {}
pub fn test_assignment_throw_17() {}
pub fn test_constexpr_assign_imp_18(v: Ptr<()>, new_value: Ptr<()>) {
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_19(v: Ptr<()>, new_value: Ptr<()>) {
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_20(v: Ptr<()>, new_value: Ptr<()>) {
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_21(v: Ptr<()>, new_value: Ptr<()>) {
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_22(v: Ptr<()>, new_value: Ptr<()>) {
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_assign_imp_23(v: Ptr<()>, new_value: Ptr<NonTrivialCopyAssign>) {
    let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_assignment_trivial_24() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_assignable<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_18(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_18(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_assign_imp_19(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_assign_imp_20(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn operator_eq_25(x: Ptr<NonTrivialCopyAssign>, y: Ptr<NonTrivialCopyAssign>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(4)]
pub struct NonTrivialCopyAssign {
    #[offset(0)]
    pub i: (),
}
impl NonTrivialCopyAssign {
    pub fn new(mut ii: ()) -> Self {
        Self { i: ii }
    }
    pub fn copy_from(other: Ptr<NonTrivialCopyAssign>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
}
impl std::cmp::PartialEq for NonTrivialCopyAssign {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_25(
                Rc::new(RefCell::new(NonTrivialCopyAssign { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(NonTrivialCopyAssign { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialCopyAssign {}
impl Clone for NonTrivialCopyAssign {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivialCopyAssign> =
            Rc::new(RefCell::new(NonTrivialCopyAssign { i: self.i.clone() }));
        NonTrivialCopyAssign::copy_from(__src.as_pointer())
    }
}
impl Default for NonTrivialCopyAssign {
    fn default() -> Self {
        { NonTrivialCopyAssign { i: 0_() } }
    }
}
pub fn test_constexpr_copy_assignment_non_trivial_26() {
    const _: () = assert!(true, "!std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "!std::is_trivially_copy_assignable<V>::value");;
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
        let _new_value: Value<NonTrivialCopyAssign> =
            Rc::new(RefCell::new(NonTrivialCopyAssign::new({ 5_() })));
        test_constexpr_assign_imp_23(_v.as_pointer(), _new_value.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _new_value: Value<NonTrivialCopyAssign> =
            Rc::new(RefCell::new(NonTrivialCopyAssign::new({ 5_() })));
        test_constexpr_assign_imp_23(_v.as_pointer(), _new_value.as_pointer())
    });
}
pub fn non_constexpr_test_27() {
    ({ test_copy_assignment_empty_empty_2() });
    ({ test_copy_assignment_non_empty_empty_3() });
    ({ test_copy_assignment_empty_non_empty_4() });
    ({ test_assignment_throw_17() });
}
pub fn cxx17_constexpr_test_28() -> () {
    ({ test_copy_assignment_sfinae_1() });
    ({ test_copy_assignment_not_noexcept_0() });
    ({ test_constexpr_copy_assignment_trivial_24() });
    return true;
}
pub fn cxx20_constexpr_test_29() -> () {
    ({ test_copy_assignment_same_index_5() });
    ({ test_copy_assignment_different_index_14() });
    ({ test_constexpr_copy_assignment_non_trivial_26() });
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
    ({ non_constexpr_test_27() });
    ({ cxx17_constexpr_test_28() });
    ({ cxx20_constexpr_test_29() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0_();
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
        (*self)
            .with(|__s| __s.copy_assign.clone())
            .with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<CopyAssign>) -> Ptr<CopyAssign> {
        field!((*self), value).write({ o.with(|__s| __s.value) });
        field!((*self), alive).write({ o.with(|__s| __s.alive.clone()) });
        field!((*self), copy_construct).write({ o.with(|__s| __s.copy_construct.clone()) });
        field!((*self), copy_assign).write({ o.with(|__s| __s.copy_assign.clone()) });
        field!((*self), move_construct).write({ o.with(|__s| __s.move_construct.clone()) });
        field!((*self), move_assign).write({ o.with(|__s| __s.move_assign.clone()) });
        field!(o, value).write(-1_());
        (*self)
            .with(|__s| __s.move_assign.clone())
            .with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn destructor(&self) {
        (*self)
            .with(|__s| __s.alive.clone())
            .with_mut(|__v| __v.prefix_dec());
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
        field!(that, value).write(-1_());
        return (*self).clone();
    }
}
pub trait anon_10Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_10Impl for Ptr<anon_10> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ({ get_11(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_12Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_12Impl for Ptr<anon_12> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ({ get_13(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_15Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_15Impl for Ptr<anon_15> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: (({ get_9(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_16Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_16Impl for Ptr<anon_16> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: ({ get_11(v.as_pointer()) }).with(|__s| __s.value),
        };
    }
}
pub trait anon_6Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_6Impl for Ptr<anon_6> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_int_ {
            index: unimplemented!(),
            value: (({ get_7(v.as_pointer()) }).read()),
        };
    }
}
pub trait anon_8Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_8Impl for Ptr<anon_8> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        return Result_long_ {
            index: unimplemented!(),
            value: (({ get_9(v.as_pointer()) }).read()),
        };
    }
}
pub fn __cpp2rust_init_globals() {}
