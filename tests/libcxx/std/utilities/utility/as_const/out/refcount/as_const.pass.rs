extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub i: (),
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_1(x: Ptr<S>, y: Ptr<S>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
pub fn test_2(t: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const                  (t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<               T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const          T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<volatile       T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const volatile T>(t))>::type>::value"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_3(t: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const                  (t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<               T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const          T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<volatile       T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const volatile T>(t))>::type>::value"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_4(t: Ptr<S>) {
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const                  (t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<               T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const          T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<volatile       T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const volatile T>(t))>::type>::value"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    let i: Value<()> = Rc::new(RefCell::new(3_()));
    let d: Value<()> = Rc::new(RefCell::new(4_f64));
    let s: Value<S> = Rc::new(RefCell::new(S { i: 2_() }));
    ({ test_2(i.as_pointer()) });
    ({ test_3(d.as_pointer()) });
    ({ test_4(s.as_pointer()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
