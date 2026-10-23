extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct ThrowsMove {}
impl ThrowsMove {
    pub fn move_from(_a0: Ptr<ThrowsMove>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCopy {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub value: (),
}
impl MoveOnly {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct MoveOnlyNT {
    #[offset(0)]
    pub value: (),
}
impl MoveOnlyNT {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn move_from(other: Ptr<MoveOnlyNT>) -> Self {
        let __this: MoveOnlyNT = Self {
            value: other.with(|__s| __s.value),
        };
        field!(other, value).write(-1_());
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct NTMove {
    #[offset(0)]
    pub value: (),
}
impl NTMove {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn move_from(that: Ptr<NTMove>) -> Self {
        let __this: NTMove = Self {
            value: that.with(|__s| __s.value),
        };
        field!(that, value).write(-1_());
        __this
    }
}
const _: () = assert!(true, "!std::is_trivially_move_constructible<NTMove>::value");
const _: () = assert!(true, "std::is_move_constructible<NTMove>::value");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct TMove {
    #[offset(0)]
    pub value: (),
}
impl TMove {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn move_from(_a0: Ptr<TMove>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
const _: () = assert!(true, "std::is_trivially_move_constructible<TMove>::value");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct TMoveNTCopy {
    #[offset(0)]
    pub value: (),
}
impl TMoveNTCopy {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn copy_from(that: Ptr<TMoveNTCopy>) -> Self {
        Self {
            value: that.with(|__s| __s.value),
        }
    }
    pub fn move_from(_a0: Ptr<TMoveNTCopy>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
impl Clone for TMoveNTCopy {
    fn clone(&self) -> Self {
        let __src: Value<TMoveNTCopy> = Rc::new(RefCell::new(TMoveNTCopy {
            value: self.value.clone(),
        }));
        TMoveNTCopy::copy_from(__src.as_pointer())
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_move_constructible<TMoveNTCopy>::value"
);
pub fn test_move_noexcept_0() {
    {
        const _: () = assert!(true, "std::is_nothrow_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_move_constructible<V>::value");;
    }
}
pub fn test_move_ctor_sfinae_1() {
    {
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_move_constructible<V>::value");;
        const _: () = assert!(true, "std::is_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    }
}
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct Result_TMove_ {
    #[offset(0)]
    pub index: (),
    #[offset(8)]
    #[byte_size(4)]
    pub value: TMove,
}
impl Result_TMove_ {
    pub fn move_from(_a0: Ptr<Result_TMove_>) -> Self {
        Self {
            index: { (*_a0.upgrade().deref()).index },
            value: TMove::move_from({ field_ptr!(_a0, value) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Result_TMoveNTCopy_ {
    #[offset(0)]
    pub index: (),
    #[offset(8)]
    #[byte_size(4)]
    pub value: TMoveNTCopy,
}
pub fn test_move_ctor_basic_2() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let test: Value<anon_3> = Rc::new(RefCell::new(<anon_3>::default()));
        let mut result: Result_int_ = ({ anon_3Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_5> = Rc::new(RefCell::new(<anon_5>::default()));
        let mut result: Result_long_ = ({ anon_5Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value == 42");;
    }
    {
        let test: Value<anon_7> = Rc::new(RefCell::new(<anon_7>::default()));
        let mut result: Result_TMove_ = ({ anon_7Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
    {
        let test: Value<anon_9> = Rc::new(RefCell::new(<anon_9>::default()));
        let mut result: Result_TMove_ = ({ anon_9Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
    {
        let test: Value<anon_11> = Rc::new(RefCell::new(<anon_11>::default()));
        let mut result: Result_TMoveNTCopy_ = ({ anon_11Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 0");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
    {
        let test: Value<anon_13> = Rc::new(RefCell::new(<anon_13>::default()));
        let mut result: Result_TMoveNTCopy_ = ({ anon_13Impl::operator_call(&test.as_pointer()) });
        const _: () = assert!(true, "result.index == 1");;
        const _: () = assert!(true, "result.value.value == 42");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_5 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_9 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_11 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct anon_13 {}
pub fn test_move_ctor_valueless_by_exception_15() {}
pub fn test_constexpr_ctor_imp_16(v: Ptr<()>) {
    let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_ctor_imp_17(v: Ptr<()>) {
    let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_ctor_imp_18(v: Ptr<()>) {
    let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_ctor_imp_19(v: Ptr<()>) {
    let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_ctor_imp_20(v: Ptr<()>) {
    let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_ctor_imp_21(v: Ptr<()>) {
    let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_move_ctor_trivial_22() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(42_()));
        test_constexpr_ctor_imp_16(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_ctor_imp_17(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(101_()));
        test_constexpr_ctor_imp_18(_v.as_pointer())
    });
}
pub fn operator_eq_23(x: Ptr<NonTrivialMoveCtor>, y: Ptr<NonTrivialMoveCtor>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(4)]
pub struct NonTrivialMoveCtor {
    #[offset(0)]
    pub i: (),
}
impl NonTrivialMoveCtor {
    pub fn new(mut ii: ()) -> Self {
        Self { i: ii }
    }
    pub fn move_from(other: Ptr<NonTrivialMoveCtor>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
}
impl std::cmp::PartialEq for NonTrivialMoveCtor {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_23(
                Rc::new(RefCell::new(NonTrivialMoveCtor { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(NonTrivialMoveCtor { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialMoveCtor {}
impl Default for NonTrivialMoveCtor {
    fn default() -> Self {
        { NonTrivialMoveCtor { i: 0_() } }
    }
}
pub fn test_constexpr_move_ctor_non_trivial_24() {
    const _: () = assert!(true, "!std::is_trivially_move_constructible<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(42_()));
        test_constexpr_ctor_imp_19(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(NonTrivialMoveCtor::new({ 5_() })));
        test_constexpr_ctor_imp_20(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(Default::default()));
        test_constexpr_ctor_imp_21(_v.as_pointer())
    });
}
pub fn non_constexpr_test_25() {
    ({ test_move_ctor_valueless_by_exception_15() });
}
pub fn cxx17_constexpr_test_26() -> () {
    ({ test_move_noexcept_0() });
    ({ test_move_ctor_sfinae_1() });
    ({ test_constexpr_move_ctor_trivial_22() });
    return true;
}
pub fn cxx20_constexpr_test_27() -> () {
    ({ test_move_ctor_basic_2() });
    ({ test_constexpr_move_ctor_non_trivial_24() });
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
    ({ non_constexpr_test_25() });
    ({ cxx17_constexpr_test_26() });
    ({ cxx20_constexpr_test_27() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0_();
}
pub trait anon_11Impl {
    fn operator_call(&self) -> Result_TMoveNTCopy_;
}
impl anon_11Impl for Ptr<anon_11> {
    fn operator_call(&self) -> Result_TMoveNTCopy_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return Result_TMoveNTCopy_ {
            index: unimplemented!(),
            value: TMoveNTCopy::move_from({
                ({ get_12((v2.as_pointer()).reinterpret_cast::<()>()) })
            }),
        };
    }
}
pub trait anon_13Impl {
    fn operator_call(&self) -> Result_TMoveNTCopy_;
}
impl anon_13Impl for Ptr<anon_13> {
    fn operator_call(&self) -> Result_TMoveNTCopy_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return Result_TMoveNTCopy_ {
            index: unimplemented!(),
            value: TMoveNTCopy::move_from({
                ({ get_14((v2.as_pointer()).reinterpret_cast::<()>()) })
            }),
        };
    }
}
pub trait anon_3Impl {
    fn operator_call(&self) -> Result_int_;
}
impl anon_3Impl for Ptr<anon_3> {
    fn operator_call(&self) -> Result_int_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return Result_int_ {
            index: unimplemented!(),
            value: ((({ get_4((v2.as_pointer()).reinterpret_cast::<()>()) }).read()) as ()),
        };
    }
}
pub trait anon_5Impl {
    fn operator_call(&self) -> Result_long_;
}
impl anon_5Impl for Ptr<anon_5> {
    fn operator_call(&self) -> Result_long_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return Result_long_ {
            index: unimplemented!(),
            value: ((({ get_6((v2.as_pointer()).reinterpret_cast::<()>()) }).read()) as ()),
        };
    }
}
pub trait anon_7Impl {
    fn operator_call(&self) -> Result_TMove_;
}
impl anon_7Impl for Ptr<anon_7> {
    fn operator_call(&self) -> Result_TMove_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return Result_TMove_ {
            index: unimplemented!(),
            value: TMove::move_from({ ({ get_8((v2.as_pointer()).reinterpret_cast::<()>()) }) }),
        };
    }
}
pub trait anon_9Impl {
    fn operator_call(&self) -> Result_TMove_;
}
impl anon_9Impl for Ptr<anon_9> {
    fn operator_call(&self) -> Result_TMove_ {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return Result_TMove_ {
            index: unimplemented!(),
            value: TMove::move_from({ ({ get_10((v2.as_pointer()).reinterpret_cast::<()>()) }) }),
        };
    }
}
pub fn __cpp2rust_init_globals() {}
