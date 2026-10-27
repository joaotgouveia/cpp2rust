extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_void_0() {
    const _: () = assert!(true, "std::is_void<T>::value");;
    const _: () = assert!(true, "std::is_void<const T>::value");;
    const _: () = assert!(true, "std::is_void<volatile T>::value");;
    const _: () = assert!(true, "std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "std::is_void_v<T>");;
    const _: () = assert!(true, "std::is_void_v<const T>");;
    const _: () = assert!(true, "std::is_void_v<volatile T>");;
    const _: () = assert!(true, "std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_1() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_2() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_3() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_4() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_5() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_6() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_7() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_8() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_9() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_10() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_11() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_12() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_13() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_14() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_15() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
}
pub fn test_is_not_void_16() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_void<const T>::value");;
    const _: () = assert!(true, "!std::is_void<volatile T>::value");;
    const _: () = assert!(true, "!std::is_void<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_void_v<T>");;
    const _: () = assert!(true, "!std::is_void_v<const T>");;
    const _: () = assert!(true, "!std::is_void_v<volatile T>");;
    const _: () = assert!(true, "!std::is_void_v<const volatile T>");;
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
    pub anon_17: (),
}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one: Enum = 1;
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
    ({ test_is_void_0() });
    ({ test_is_not_void_1() });
    ({ test_is_not_void_2() });
    ({ test_is_not_void_3() });
    ({ test_is_not_void_4() });
    ({ test_is_not_void_5() });
    ({ test_is_not_void_6() });
    ({ test_is_not_void_7() });
    ({ test_is_not_void_8() });
    ({ test_is_not_void_9() });
    ({ test_is_not_void_10() });
    ({ test_is_not_void_11() });
    ({ test_is_not_void_12() });
    ({ test_is_not_void_13() });
    ({ test_is_not_void_14() });
    ({ test_is_not_void_15() });
    ({ test_is_not_void_16() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
