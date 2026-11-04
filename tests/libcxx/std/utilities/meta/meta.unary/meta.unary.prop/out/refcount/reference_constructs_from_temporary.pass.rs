extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TrivialNotNoexcept {}
impl CppMove for TrivialNotNoexcept {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
impl CppMove for Empty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NotEmpty {}
impl CppMove for NotEmpty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct Union {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl Union {}
impl CppMove for Union {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Union {
    fn default() -> Self {
        Union {
            __bytes: Box::from([0u8; 1]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bit_zero {
    #[offset(0)]
    pub anon_0: i32,
}
impl CppMove for bit_zero {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonPODClass {}
impl NonPODClass {}
impl CppMove for NonPODClass {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type Enum = u32;
pub const Enum_EV: Enum = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Base {
    #[offset(0)]
    pub e: Enum,
    #[offset(4)]
    pub i: i32,
    #[offset(8)]
    pub f: f32,
    #[offset(16)]
    #[byte_size(8)]
    pub p: Ptr<NonPODClass>,
}
impl CppMove for Base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Derived {}
impl CppMove for Derived {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertsToRef_int__int_ref_ {
    #[offset(0)]
    pub obj: i32,
}
impl CppMove for ConvertsToRef_int__int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for ConvertsToRef_int__int_ref_ {
    fn default() -> Self {
        { ConvertsToRef_int__int_ref_ { obj: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertsToRef_int__int_refref_ {
    #[offset(0)]
    pub obj: i32,
}
impl CppMove for ConvertsToRef_int__int_refref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for ConvertsToRef_int__int_refref_ {
    fn default() -> Self {
        { ConvertsToRef_int__int_refref_ { obj: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertsToRef_int__const_int_ref_ {
    #[offset(0)]
    pub obj: i32,
}
impl CppMove for ConvertsToRef_int__const_int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for ConvertsToRef_int__const_int_ref_ {
    fn default() -> Self {
        { ConvertsToRef_int__const_int_ref_ { obj: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertsToRef_long__long_ref_ {
    #[offset(0)]
    pub obj: i64,
}
impl CppMove for ConvertsToRef_long__long_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for ConvertsToRef_long__long_ref_ {
    fn default() -> Self {
        { ConvertsToRef_long__long_ref_ { obj: 0_i64 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertsToRefPrivate_long__long_ref_ {
    #[offset(0)]
    obj: i64,
}
impl CppMove for ConvertsToRefPrivate_long__long_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for ConvertsToRefPrivate_long__long_ref_ {
    fn default() -> Self {
        { ConvertsToRefPrivate_long__long_ref_ { obj: 0_i64 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitConversionRvalueRef {}
impl CppMove for ExplicitConversionRvalueRef {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitConversionRef {}
impl CppMove for ExplicitConversionRef {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_reference_constructs_from_temporary_1() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_3.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_4() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_5.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_6() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_7.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_8() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_9.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_10() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_11.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_12() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_13.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_14() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_16.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_17() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_18.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_19() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_20.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_21() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_22.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_23() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_24.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_25() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_26.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_27() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_28.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_29() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_30.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_31() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_32.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_33() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_34.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_35() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_36.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_37() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_38.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_39() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_40.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_41() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_42.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_43() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_44.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_45() {
    assert!(((value_15.with(|rc| *rc.borrow()) as i32) == (true as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_46.with(|rc| *rc.borrow()) as i32)
            == (true as i32))
    );
}
pub fn test_reference_constructs_from_temporary_47() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_48.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_49() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_50.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_reference_constructs_from_temporary_51() {
    assert!(((value_2.with(|rc| *rc.borrow()) as i32) == (false as i32)));
    assert!(
        ((reference_constructs_from_temporary_v_52.with(|rc| *rc.borrow()) as i32)
            == (false as i32))
    );
}
pub fn test_53() -> bool {
    ({ test_reference_constructs_from_temporary_1() });
    ({ test_reference_constructs_from_temporary_1() });
    ({ test_reference_constructs_from_temporary_4() });
    ({ test_reference_constructs_from_temporary_6() });
    ({ test_reference_constructs_from_temporary_8() });
    ({ test_reference_constructs_from_temporary_10() });
    ({ test_reference_constructs_from_temporary_12() });
    ({ test_reference_constructs_from_temporary_14() });
    ({ test_reference_constructs_from_temporary_17() });
    ({ test_reference_constructs_from_temporary_19() });
    assert!((is_constructible_v_54.with(|rc| *rc.borrow())));
    ({ test_reference_constructs_from_temporary_21() });
    assert!((is_constructible_v_55.with(|rc| *rc.borrow())));
    ({ test_reference_constructs_from_temporary_23() });
    assert!((is_constructible_v_56.with(|rc| *rc.borrow())));
    ({ test_reference_constructs_from_temporary_25() });
    assert!((is_constructible_v_57.with(|rc| *rc.borrow())));
    ({ test_reference_constructs_from_temporary_27() });
    ({ test_reference_constructs_from_temporary_29() });
    ({ test_reference_constructs_from_temporary_31() });
    ({ test_reference_constructs_from_temporary_33() });
    ({ test_reference_constructs_from_temporary_35() });
    ({ test_reference_constructs_from_temporary_37() });
    ({ test_reference_constructs_from_temporary_39() });
    ({ test_reference_constructs_from_temporary_41() });
    ({ test_reference_constructs_from_temporary_10() });
    ({ test_reference_constructs_from_temporary_43() });
    ({ test_reference_constructs_from_temporary_45() });
    ({ test_reference_constructs_from_temporary_47() });
    ({ test_reference_constructs_from_temporary_49() });
    ({ test_reference_constructs_from_temporary_51() });
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
    ({ test_53() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait AImpl {
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!()
    }
}
impl AImpl for Ptr<A> {}
pub trait ConvertsToRefPrivate_long__long_ref_Impl {
    fn to_pmuti64(&self) -> Ptr<i64> {
        unimplemented!()
    }
}
impl ConvertsToRefPrivate_long__long_ref_Impl for Ptr<ConvertsToRefPrivate_long__long_ref_> {}
pub trait ConvertsToRef_int__const_int_ref_Impl {
    fn to_pconsti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl ConvertsToRef_int__const_int_ref_Impl for Ptr<ConvertsToRef_int__const_int_ref_> {}
pub trait ConvertsToRef_int__int_ref_Impl {
    fn to_pmuti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl ConvertsToRef_int__int_ref_Impl for Ptr<ConvertsToRef_int__int_ref_> {}
pub trait ConvertsToRef_int__int_refref_Impl {
    fn to_pmuti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl ConvertsToRef_int__int_refref_Impl for Ptr<ConvertsToRef_int__int_refref_> {}
pub trait ConvertsToRef_long__long_ref_Impl {
    fn to_pmuti64(&self) -> Ptr<i64> {
        unimplemented!()
    }
}
impl ConvertsToRef_long__long_ref_Impl for Ptr<ConvertsToRef_long__long_ref_> {}
pub trait ExplicitConversionRefImpl {
    fn to_i32(&self) -> i32 {
        unimplemented!()
    }
    fn to_pmuti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl ExplicitConversionRefImpl for Ptr<ExplicitConversionRef> {}
pub trait ExplicitConversionRvalueRefImpl {
    fn to_i32(&self) -> i32 {
        unimplemented!()
    }
    fn to_pmuti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl ExplicitConversionRvalueRefImpl for Ptr<ExplicitConversionRvalueRef> {}
pub fn __cpp2rust_init_globals() {}
