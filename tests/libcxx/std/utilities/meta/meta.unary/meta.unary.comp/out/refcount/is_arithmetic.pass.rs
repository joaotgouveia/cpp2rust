extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_arithmetic_0() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_1() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_2() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_3() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_4() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_5() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_6() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_7() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_8() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_9() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_10() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_arithmetic_11() {
    const _: () = assert!(true, "std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_12() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_13() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_14() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_15() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_16() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_17() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_18() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_19() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_20() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_21() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_22() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_23() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_24() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_25() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_26() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_27() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_28() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_29() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
pub fn test_is_not_arithmetic_30() {
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic_v<T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<volatile T>");;
    const _: () = assert!(true, "!std::is_arithmetic_v<const volatile T>");;
}
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
    pub anon_31: (),
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
    ({ test_is_arithmetic_0() });
    ({ test_is_arithmetic_1() });
    ({ test_is_arithmetic_2() });
    ({ test_is_arithmetic_3() });
    ({ test_is_arithmetic_4() });
    ({ test_is_arithmetic_5() });
    ({ test_is_arithmetic_6() });
    ({ test_is_arithmetic_7() });
    ({ test_is_arithmetic_8() });
    ({ test_is_arithmetic_9() });
    ({ test_is_arithmetic_10() });
    ({ test_is_arithmetic_11() });
    ({ test_is_not_arithmetic_12() });
    ({ test_is_not_arithmetic_13() });
    ({ test_is_not_arithmetic_14() });
    ({ test_is_not_arithmetic_15() });
    ({ test_is_not_arithmetic_16() });
    ({ test_is_not_arithmetic_17() });
    ({ test_is_not_arithmetic_18() });
    ({ test_is_not_arithmetic_19() });
    ({ test_is_not_arithmetic_20() });
    ({ test_is_not_arithmetic_21() });
    ({ test_is_not_arithmetic_22() });
    ({ test_is_not_arithmetic_23() });
    ({ test_is_not_arithmetic_24() });
    ({ test_is_not_arithmetic_25() });
    ({ test_is_not_arithmetic_26() });
    ({ test_is_not_arithmetic_27() });
    ({ test_is_not_arithmetic_28() });
    ({ test_is_not_arithmetic_29() });
    ({ test_is_not_arithmetic_30() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
