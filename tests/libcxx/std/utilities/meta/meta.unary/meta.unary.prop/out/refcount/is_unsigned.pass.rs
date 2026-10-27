extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_unsigned_0() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_unsigned_1() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_unsigned_2() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_unsigned_3() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_unsigned_4() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_unsigned_5() {
    const _: () = assert!(true, "std::is_unsigned<T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "std::is_unsigned_v<T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_6() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_7() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_8() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_9() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_10() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_11() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_12() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_13() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_14() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_15() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_16() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_17() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_18() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_19() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_20() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_21() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_22() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_23() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_24() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_25() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_26() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_27() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_28() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_29() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_30() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_31() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_32() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
pub fn test_is_not_unsigned_33() {
    const _: () = assert!(true, "!std::is_unsigned<T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_unsigned_v<T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<volatile T>");;
    const _: () = assert!(true, "!std::is_unsigned_v<const volatile T>");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
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
    pub anon_34: (),
}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one: Enum = 1;
pub type EnumSigned = ();
pub const EnumSigned_two: EnumSigned = 0;
pub type EnumUnsigned = ();
pub const EnumUnsigned_three: EnumUnsigned = 0;
pub type EnumClass = ();
pub const EnumClass_zero: EnumClass = 0;
pub const EnumClass_one: EnumClass = 1;
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
    ({ test_is_not_unsigned_6() });
    ({ test_is_not_unsigned_7() });
    ({ test_is_not_unsigned_8() });
    ({ test_is_not_unsigned_9() });
    ({ test_is_not_unsigned_10() });
    ({ test_is_not_unsigned_11() });
    ({ test_is_not_unsigned_12() });
    ({ test_is_not_unsigned_13() });
    ({ test_is_not_unsigned_14() });
    ({ test_is_not_unsigned_15() });
    ({ test_is_not_unsigned_16() });
    ({ test_is_not_unsigned_17() });
    ({ test_is_not_unsigned_18() });
    ({ test_is_not_unsigned_19() });
    ({ test_is_not_unsigned_20() });
    ({ test_is_not_unsigned_21() });
    ({ test_is_not_unsigned_22() });
    ({ test_is_not_unsigned_23() });
    ({ test_is_not_unsigned_24() });
    ({ test_is_not_unsigned_25() });
    ({ test_is_not_unsigned_26() });
    ({ test_is_not_unsigned_27() });
    ({ test_is_not_unsigned_28() });
    ({ test_is_not_unsigned_29() });
    ({ test_is_not_unsigned_30() });
    ({ test_is_not_unsigned_31() });
    ({ test_is_not_unsigned_32() });
    ({ test_is_unsigned_0() });
    ({ test_is_unsigned_1() });
    ({ test_is_unsigned_2() });
    ({ test_is_unsigned_3() });
    ({ test_is_unsigned_4() });
    ({ test_is_unsigned_2() });
    ({ test_is_unsigned_5() });
    ({ test_is_not_unsigned_33() });
    return 0_();
}
pub trait ClassImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl ClassImpl for Ptr<Class> {}
pub fn __cpp2rust_init_globals() {}
