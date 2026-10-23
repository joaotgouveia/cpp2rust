extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_has_virtual_destructor_0() {
    const _: () = assert!(true, "std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor<const volatile T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_virtual_destructor_1() {
    const _: () = assert!(true, "std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor<const volatile T>::value");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_2() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_3() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_4() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_5() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_6() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_7() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_8() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_9() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_10() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_11() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_12() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
}
pub fn test_has_not_virtual_destructor_13() {
    const _: () = assert!(true, "!std::has_virtual_destructor<T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<const T>::value");;
    const _: () = assert!(true, "!std::has_virtual_destructor<volatile T>::value");;
    const _: () = assert!(
        true,
        "!std::has_virtual_destructor<const volatile T>::value"
    );;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<volatile T>");;
    const _: () = assert!(true, "!std::has_virtual_destructor_v<const volatile T>");;
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
    pub anon_14: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
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
    ({ test_has_not_virtual_destructor_2() });
    ({ test_has_not_virtual_destructor_3() });
    ({ test_has_not_virtual_destructor_4() });
    ({ test_has_not_virtual_destructor_5() });
    ({ test_has_not_virtual_destructor_6() });
    ({ test_has_not_virtual_destructor_7() });
    ({ test_has_not_virtual_destructor_8() });
    ({ test_has_not_virtual_destructor_9() });
    ({ test_has_not_virtual_destructor_10() });
    ({ test_has_not_virtual_destructor_11() });
    ({ test_has_not_virtual_destructor_12() });
    ({ test_has_not_virtual_destructor_13() });
    ({ test_has_virtual_destructor_0() });
    ({ test_has_virtual_destructor_1() });
    return 0_();
}
pub trait AImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl AImpl for Ptr<A> {}
pub fn __cpp2rust_init_globals() {}
