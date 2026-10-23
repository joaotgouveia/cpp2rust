extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_nothrow_assignable_0() {
    const _: () = assert!(true, "( std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "( std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_nothrow_assignable_1() {
    const _: () = assert!(true, "( std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "( std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_nothrow_assignable_2() {
    const _: () = assert!(true, "( std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "( std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_not_nothrow_assignable_3() {
    const _: () = assert!(true, "(!std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_not_nothrow_assignable_4() {
    const _: () = assert!(true, "(!std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_not_nothrow_assignable_5() {
    const _: () = assert!(true, "(!std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_not_nothrow_assignable_6() {
    const _: () = assert!(true, "(!std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_nothrow_assignable_v<T, U>)");;
}
pub fn test_is_not_nothrow_assignable_7() {
    const _: () = assert!(true, "(!std::is_nothrow_assignable<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_nothrow_assignable_v<T, U>)");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
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
    ({ test_is_nothrow_assignable_0() });
    ({ test_is_nothrow_assignable_1() });
    ({ test_is_nothrow_assignable_2() });
    ({ test_is_not_nothrow_assignable_3() });
    ({ test_is_not_nothrow_assignable_4() });
    ({ test_is_not_nothrow_assignable_5() });
    ({ test_is_not_nothrow_assignable_6() });
    ({ test_is_not_nothrow_assignable_7() });
    return 0_();
}
pub trait BImpl {
    fn operator_assign_1(&self, mut _a0: A) {
        unimplemented!()
    }
}
impl BImpl for Ptr<B> {}
pub trait CImpl {
    fn copy_assign(&self, _a0: Ptr<C>) {
        unimplemented!()
    }
}
impl CImpl for Ptr<C> {}
pub fn __cpp2rust_init_globals() {}
