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
pub struct B {
    #[offset(0)]
    #[byte_size(1)]
    pub a: A,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct D {
    #[offset(0)]
    #[byte_size(1)]
    pub c: C,
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
    const _: () = assert!(true, "(std::is_nothrow_convertible<int, double>::value)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible<int, char*>::value)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible<A, B>::value)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible<D, C>::value)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<int, double>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int, char*>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<A, B>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<D, C>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<const void, void>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<volatile void, void>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<void, const void>)");;
    const _: () = assert!(true, "(std::is_nothrow_convertible_v<void, volatile void>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[], double[]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[], int[]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[10], int[10]>)");;
    const _: () = assert!(
        true,
        "!(std::is_nothrow_convertible_v<int[10], double[10]>)"
    );;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[5], double[10]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<int[10], A[10]>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<V, V>)");;
    const _: () = assert!(true, "!(std::is_nothrow_convertible_v<V, I>)");;
    return 0_();
}
pub trait BImpl {
    fn to_A(&self) -> A;
}
impl BImpl for Ptr<B> {
    fn to_A(&self) -> A {
        return (*self).with(|__s| __s.a.clone());
    }
}
pub trait DImpl {
    fn to_C(&self) -> C;
}
impl DImpl for Ptr<D> {
    fn to_C(&self) -> C {
        return (*self).with(|__s| __s.c.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
