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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitConversionRvalueRef {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitConversionRef {}
pub fn test_is_nothrow_copy_constructible_1() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_2() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_3() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_4() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_5() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_6() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_7() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_is_nothrow_copy_constructible_8() {
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_nothrow_copy_constructible_v<const T>");;
}
pub fn test_has_not_nothrow_copy_constructor_9() {
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible<volatile T>::value"
    );;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<const T>");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<volatile T>");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible_v<const volatile T>"
    );;
}
pub fn test_has_not_nothrow_copy_constructor_10() {
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible<volatile T>::value"
    );;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<const T>");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<volatile T>");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible_v<const volatile T>"
    );;
}
pub fn test_has_not_nothrow_copy_constructor_11() {
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible<const T>::value");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible<volatile T>::value"
    );;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<const T>");;
    const _: () = assert!(true, "!std::is_nothrow_copy_constructible_v<volatile T>");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_copy_constructible_v<const volatile T>"
    );;
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
    ({ test_has_not_nothrow_copy_constructor_9() });
    ({ test_has_not_nothrow_copy_constructor_10() });
    ({ test_has_not_nothrow_copy_constructor_11() });
    ({ test_is_nothrow_copy_constructible_1() });
    ({ test_is_nothrow_copy_constructible_2() });
    ({ test_is_nothrow_copy_constructible_3() });
    ({ test_is_nothrow_copy_constructible_4() });
    ({ test_is_nothrow_copy_constructible_5() });
    ({ test_is_nothrow_copy_constructible_6() });
    ({ test_is_nothrow_copy_constructible_7() });
    ({ test_is_nothrow_copy_constructible_8() });
    return 0_();
}
pub trait AImpl {
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!()
    }
}
impl AImpl for Ptr<A> {}
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
