extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_trivially_copyable_0() {
    const _: () = assert!(true, "std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_trivially_copyable_1() {
    const _: () = assert!(true, "std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_trivially_copyable_2() {
    const _: () = assert!(true, "std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_trivially_copyable_3() {
    const _: () = assert!(true, "std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_trivially_copyable_4() {
    const _: () = assert!(true, "std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_not_trivially_copyable_5() {
    const _: () = assert!(true, "!std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_not_trivially_copyable_6() {
    const _: () = assert!(true, "!std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<const volatile T>");;
}
pub fn test_is_not_trivially_copyable_7() {
    const _: () = assert!(true, "!std::is_trivially_copyable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<const T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<volatile T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<const T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<volatile T>");;
    const _: () = assert!(true, "!std::is_trivially_copyable_v<const volatile T>");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub i_: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub i_: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
impl C {}
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
    ({ test_is_trivially_copyable_0() });
    ({ test_is_trivially_copyable_1() });
    ({ test_is_trivially_copyable_2() });
    ({ test_is_trivially_copyable_3() });
    ({ test_is_trivially_copyable_4() });
    ({ test_is_not_trivially_copyable_5() });
    ({ test_is_not_trivially_copyable_6() });
    ({ test_is_not_trivially_copyable_7() });
    return 0_();
}
pub trait BImpl {
    fn destructor(&self);
}
impl BImpl for Ptr<B> {
    fn destructor(&self) {
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
