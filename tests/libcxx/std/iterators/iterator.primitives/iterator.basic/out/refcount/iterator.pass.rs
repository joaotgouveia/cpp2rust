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
pub struct A {}
pub fn test2_0() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<typename It::pointer, T*>::value)");;
    const _: () = assert!(true, "(std::is_same<typename It::reference, T&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::forward_iterator_tag>::value)"
    );;
}
pub fn test3_1() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, short>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<typename It::pointer, T*>::value)");;
    const _: () = assert!(true, "(std::is_same<typename It::reference, T&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::bidirectional_iterator_tag>::value)"
    );;
}
pub fn test4_2() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::pointer, const T*>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<typename It::reference, T&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::random_access_iterator_tag>::value)"
    );;
}
pub fn test5_3() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, long>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::pointer, const T*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::reference, const T&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::input_iterator_tag>::value)"
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
    ({ test2_0() });
    ({ test3_1() });
    ({ test4_2() });
    ({ test5_3() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
