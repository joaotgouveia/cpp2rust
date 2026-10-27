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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct B {}
pub fn test_is_assignable_0() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_assignable_1() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_assignable_2() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_assignable_3() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_assignable_4() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_assignable_5() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_assignable_6() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub fn test_is_not_assignable_7() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub fn test_is_not_assignable_8() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub fn test_is_not_assignable_9() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub fn test_is_not_assignable_10() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub fn test_is_not_assignable_11() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub fn test_is_not_assignable_12() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct E {}
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
    ({ test_is_assignable_0() });
    ({ test_is_assignable_1() });
    ({ test_is_assignable_2() });
    ({ test_is_assignable_3() });
    ({ test_is_assignable_4() });
    ({ test_is_assignable_5() });
    ({ test_is_not_assignable_7() });
    ({ test_is_not_assignable_8() });
    ({ test_is_not_assignable_9() });
    ({ test_is_not_assignable_10() });
    ({ test_is_not_assignable_11() });
    ({ test_is_not_assignable_12() });
    ({ test_is_assignable_6() });
    return 0_();
}
pub trait BImpl {
    fn operator_assign_1(&self, mut _a0: A) {
        unimplemented!()
    }
}
impl BImpl for Ptr<B> {}
pub trait EImpl {
    fn operator_assign_1(&self, mut _a0: ()) -> C {
        unimplemented!()
    }
}
impl EImpl for Ptr<E> {}
pub fn __cpp2rust_init_globals() {}
