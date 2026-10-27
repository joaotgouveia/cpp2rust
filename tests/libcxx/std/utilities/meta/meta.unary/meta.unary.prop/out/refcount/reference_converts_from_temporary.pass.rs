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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NotEmpty {}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct Union {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl Union {}
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
    pub anon_0: (),
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonPODClass {}
impl NonPODClass {}
pub type Enum = ();
pub const Enum_EV: Enum = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Base {
    #[offset(0)]
    pub e: Enum,
    #[offset(4)]
    pub i: (),
    #[offset(8)]
    pub f: (),
    #[offset(16)]
    #[byte_size(8)]
    pub p: Ptr<NonPODClass>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Derived {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertsToRef_int__int_ref_ {
    #[offset(0)]
    pub obj: (),
}
impl Default for ConvertsToRef_int__int_ref_ {
    fn default() -> Self {
        {
            ConvertsToRef_int__int_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertsToRef_int__int_refref_ {
    #[offset(0)]
    pub obj: (),
}
impl Default for ConvertsToRef_int__int_refref_ {
    fn default() -> Self {
        {
            ConvertsToRef_int__int_refref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ConvertsToRef_int__const_int_ref_ {
    #[offset(0)]
    pub obj: (),
}
impl Default for ConvertsToRef_int__const_int_ref_ {
    fn default() -> Self {
        {
            ConvertsToRef_int__const_int_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertsToRef_long__long_ref_ {
    #[offset(0)]
    pub obj: (),
}
impl Default for ConvertsToRef_long__long_ref_ {
    fn default() -> Self {
        {
            ConvertsToRef_long__long_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConvertsToRefPrivate_long__long_ref_ {
    #[offset(0)]
    obj: (),
}
impl Default for ConvertsToRefPrivate_long__long_ref_ {
    fn default() -> Self {
        {
            ConvertsToRefPrivate_long__long_ref_ {
                obj: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitConversionRvalueRef {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitConversionRef {}
pub fn test_reference_converts_from_temporary_1() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_2() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_3() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_4() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_5() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_6() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_7() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_8() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_9() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_10() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_11() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_12() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_13() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_14() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_15() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_16() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_17() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_18() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_19() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_20() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_21() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_22() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_23() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_24() {
    unimplemented!();
    unimplemented!();
}
pub fn test_reference_converts_from_temporary_25() {
    unimplemented!();
    unimplemented!();
}
pub fn test_26() -> () {
    ({ test_reference_converts_from_temporary_1() });
    ({ test_reference_converts_from_temporary_1() });
    ({ test_reference_converts_from_temporary_2() });
    ({ test_reference_converts_from_temporary_3() });
    ({ test_reference_converts_from_temporary_4() });
    ({ test_reference_converts_from_temporary_5() });
    ({ test_reference_converts_from_temporary_6() });
    ({ test_reference_converts_from_temporary_7() });
    ({ test_reference_converts_from_temporary_8() });
    ({ test_reference_converts_from_temporary_9() });
    unimplemented!();
    ({ test_reference_converts_from_temporary_10() });
    unimplemented!();
    ({ test_reference_converts_from_temporary_11() });
    unimplemented!();
    ({ test_reference_converts_from_temporary_12() });
    unimplemented!();
    ({ test_reference_converts_from_temporary_13() });
    ({ test_reference_converts_from_temporary_14() });
    ({ test_reference_converts_from_temporary_15() });
    ({ test_reference_converts_from_temporary_16() });
    ({ test_reference_converts_from_temporary_17() });
    ({ test_reference_converts_from_temporary_18() });
    ({ test_reference_converts_from_temporary_19() });
    ({ test_reference_converts_from_temporary_20() });
    ({ test_reference_converts_from_temporary_5() });
    ({ test_reference_converts_from_temporary_21() });
    ({ test_reference_converts_from_temporary_22() });
    ({ test_reference_converts_from_temporary_23() });
    ({ test_reference_converts_from_temporary_24() });
    ({ test_reference_converts_from_temporary_25() });
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
    ({ test_26() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait AImpl {
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!()
    }
}
impl AImpl for Ptr<A> {}
pub trait ConvertsToRefPrivate_long__long_ref_Impl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ConvertsToRefPrivate_long__long_ref_Impl for Ptr<ConvertsToRefPrivate_long__long_ref_> {}
pub trait ConvertsToRef_int__const_int_ref_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ConvertsToRef_int__const_int_ref_Impl for Ptr<ConvertsToRef_int__const_int_ref_> {}
pub trait ConvertsToRef_int__int_ref_Impl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ConvertsToRef_int__int_ref_Impl for Ptr<ConvertsToRef_int__int_ref_> {}
pub trait ConvertsToRef_int__int_refref_Impl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ConvertsToRef_int__int_refref_Impl for Ptr<ConvertsToRef_int__int_refref_> {}
pub trait ConvertsToRef_long__long_ref_Impl {
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ConvertsToRef_long__long_ref_Impl for Ptr<ConvertsToRef_long__long_ref_> {}
pub trait ExplicitConversionRefImpl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ExplicitConversionRefImpl for Ptr<ExplicitConversionRef> {}
pub trait ExplicitConversionRvalueRefImpl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn to_pmut(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl ExplicitConversionRvalueRefImpl for Ptr<ExplicitConversionRvalueRef> {}
pub fn __cpp2rust_init_globals() {}
