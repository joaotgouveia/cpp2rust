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
pub struct P {}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct U1 {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl U1 {}
impl Default for U1 {
    fn default() -> Self {
        U1 {
            __bytes: Box::from([0u8; 1]),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct U2 {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl U2 {}
impl Default for U2 {
    fn default() -> Self {
        U2 {
            __bytes: Box::from([0u8; 1]),
        }
    }
}
pub fn test_is_final_0() {
    const _: () = assert!(true, "std::is_final<T>::value");;
    const _: () = assert!(true, "std::is_final<const T>::value");;
    const _: () = assert!(true, "std::is_final<volatile T>::value");;
    const _: () = assert!(true, "std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "std::is_final_v<T>");;
    const _: () = assert!(true, "std::is_final_v<const T>");;
    const _: () = assert!(true, "std::is_final_v<volatile T>");;
    const _: () = assert!(true, "std::is_final_v<const volatile T>");;
}
pub fn test_is_final_1() {
    const _: () = assert!(true, "std::is_final<T>::value");;
    const _: () = assert!(true, "std::is_final<const T>::value");;
    const _: () = assert!(true, "std::is_final<volatile T>::value");;
    const _: () = assert!(true, "std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "std::is_final_v<T>");;
    const _: () = assert!(true, "std::is_final_v<const T>");;
    const _: () = assert!(true, "std::is_final_v<volatile T>");;
    const _: () = assert!(true, "std::is_final_v<const volatile T>");;
}
pub fn test_is_not_final_2() {
    const _: () = assert!(true, "!std::is_final<T>::value");;
    const _: () = assert!(true, "!std::is_final<const T>::value");;
    const _: () = assert!(true, "!std::is_final<volatile T>::value");;
    const _: () = assert!(true, "!std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_final_v<T>");;
    const _: () = assert!(true, "!std::is_final_v<const T>");;
    const _: () = assert!(true, "!std::is_final_v<volatile T>");;
    const _: () = assert!(true, "!std::is_final_v<const volatile T>");;
}
pub fn test_is_not_final_3() {
    const _: () = assert!(true, "!std::is_final<T>::value");;
    const _: () = assert!(true, "!std::is_final<const T>::value");;
    const _: () = assert!(true, "!std::is_final<volatile T>::value");;
    const _: () = assert!(true, "!std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_final_v<T>");;
    const _: () = assert!(true, "!std::is_final_v<const T>");;
    const _: () = assert!(true, "!std::is_final_v<volatile T>");;
    const _: () = assert!(true, "!std::is_final_v<const volatile T>");;
}
pub fn test_is_not_final_4() {
    const _: () = assert!(true, "!std::is_final<T>::value");;
    const _: () = assert!(true, "!std::is_final<const T>::value");;
    const _: () = assert!(true, "!std::is_final<volatile T>::value");;
    const _: () = assert!(true, "!std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_final_v<T>");;
    const _: () = assert!(true, "!std::is_final_v<const T>");;
    const _: () = assert!(true, "!std::is_final_v<volatile T>");;
    const _: () = assert!(true, "!std::is_final_v<const volatile T>");;
}
pub fn test_is_not_final_5() {
    const _: () = assert!(true, "!std::is_final<T>::value");;
    const _: () = assert!(true, "!std::is_final<const T>::value");;
    const _: () = assert!(true, "!std::is_final<volatile T>::value");;
    const _: () = assert!(true, "!std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_final_v<T>");;
    const _: () = assert!(true, "!std::is_final_v<const T>");;
    const _: () = assert!(true, "!std::is_final_v<volatile T>");;
    const _: () = assert!(true, "!std::is_final_v<const volatile T>");;
}
pub fn test_is_not_final_6() {
    const _: () = assert!(true, "!std::is_final<T>::value");;
    const _: () = assert!(true, "!std::is_final<const T>::value");;
    const _: () = assert!(true, "!std::is_final<volatile T>::value");;
    const _: () = assert!(true, "!std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_final_v<T>");;
    const _: () = assert!(true, "!std::is_final_v<const T>");;
    const _: () = assert!(true, "!std::is_final_v<volatile T>");;
    const _: () = assert!(true, "!std::is_final_v<const volatile T>");;
}
pub fn test_is_not_final_7() {
    const _: () = assert!(true, "!std::is_final<T>::value");;
    const _: () = assert!(true, "!std::is_final<const T>::value");;
    const _: () = assert!(true, "!std::is_final<volatile T>::value");;
    const _: () = assert!(true, "!std::is_final<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_final_v<T>");;
    const _: () = assert!(true, "!std::is_final_v<const T>");;
    const _: () = assert!(true, "!std::is_final_v<volatile T>");;
    const _: () = assert!(true, "!std::is_final_v<const volatile T>");;
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
    ({ test_is_not_final_2() });
    ({ test_is_not_final_3() });
    ({ test_is_final_0() });
    ({ test_is_not_final_4() });
    ({ test_is_not_final_5() });
    ({ test_is_not_final_6() });
    ({ test_is_final_1() });
    ({ test_is_not_final_7() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
