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
pub fn test_reference_converts_from_temporary_1() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_4.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_5() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_6.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_7() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_8.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_9() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_10.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_11() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_12.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_13() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_14.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_15() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_17.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_18() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_19.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_20() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_21.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_22() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_23.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_24() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_25.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_26() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_27.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_28() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_29.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_30() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_31.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_32() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_33.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_34() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_35.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_36() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_37.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_38() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_39.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_40() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_41.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_42() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_43.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_44() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_45.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_46() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_47.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_48() {
    ({ cpp2rust_assert_fail_2(((value_3.with(|rc| *rc.borrow()) as i32) == (false as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_49.with(|rc| *rc.borrow()) as i32)
                == (false as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_50() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_51.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_reference_converts_from_temporary_52() {
    ({ cpp2rust_assert_fail_2(((value_16.with(|rc| *rc.borrow()) as i32) == (true as i32))) });
    ({
        cpp2rust_assert_fail_2(
            ((reference_converts_from_temporary_v_53.with(|rc| *rc.borrow()) as i32)
                == (true as i32)),
        )
    });
}
pub fn test_54() -> bool {
    ({ test_reference_converts_from_temporary_1() });
    ({ test_reference_converts_from_temporary_1() });
    ({ test_reference_converts_from_temporary_5() });
    ({ test_reference_converts_from_temporary_7() });
    ({ test_reference_converts_from_temporary_9() });
    ({ test_reference_converts_from_temporary_11() });
    ({ test_reference_converts_from_temporary_13() });
    ({ test_reference_converts_from_temporary_15() });
    ({ test_reference_converts_from_temporary_18() });
    ({ test_reference_converts_from_temporary_20() });
    ({ cpp2rust_assert_fail_2((is_constructible_v_55.with(|rc| *rc.borrow()))) });
    ({ test_reference_converts_from_temporary_22() });
    ({ cpp2rust_assert_fail_2((is_constructible_v_56.with(|rc| *rc.borrow()))) });
    ({ test_reference_converts_from_temporary_24() });
    ({ cpp2rust_assert_fail_2((is_constructible_v_57.with(|rc| *rc.borrow()))) });
    ({ test_reference_converts_from_temporary_26() });
    ({ cpp2rust_assert_fail_2((is_constructible_v_58.with(|rc| *rc.borrow()))) });
    ({ test_reference_converts_from_temporary_28() });
    ({ test_reference_converts_from_temporary_30() });
    ({ test_reference_converts_from_temporary_32() });
    ({ test_reference_converts_from_temporary_34() });
    ({ test_reference_converts_from_temporary_36() });
    ({ test_reference_converts_from_temporary_38() });
    ({ test_reference_converts_from_temporary_40() });
    ({ test_reference_converts_from_temporary_42() });
    ({ test_reference_converts_from_temporary_11() });
    ({ test_reference_converts_from_temporary_44() });
    ({ test_reference_converts_from_temporary_46() });
    ({ test_reference_converts_from_temporary_48() });
    ({ test_reference_converts_from_temporary_50() });
    ({ test_reference_converts_from_temporary_52() });
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
    ({ test_54() });
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
