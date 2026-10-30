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
pub fn source_0() -> A {
    return <A>::default();
}
pub fn csource_1() -> A {
    return <A>::default();
}
pub fn test_constexpr_forward_2() -> () {
    let x: Value<()> = Rc::new(RefCell::new(42_()));
    let mut cx: () = 101_();
    return ((((((((*x.borrow()) == 42_()) && ((*x.borrow()) == 42_()))
        && ((*x.borrow()) == 42_()))
        && ((*x.borrow()) == 42_()))
        && ((*x.borrow()) == 42_()))
        && ((*x.borrow()) == 42_()))
        && (cx == 101_()))
        && (cx == 101_());
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
    let a: Value<A> = Rc::new(RefCell::new(<A>::default()));
    let ca: Value<A> = Rc::new(RefCell::new(<A>::default()));
    (&(*a.borrow_mut()));
    (&(*ca.borrow_mut()));
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<A&>(a)), A&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<A>(a)), A&&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<A>(source())), A&&>::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A&>(a)), const A&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(a)), const A&&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(source())), const A&&>::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A&>(ca)), const A&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(ca)), const A&&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(std::forward<const A>(csource())), const A&&>::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mut i2: () = 42_();
        const _: () = assert!(true, "std::forward<int>(42) == 42");;
        const _: () = assert!(true, "std::forward<const int&>(i2) == 42");;
        const _: () = assert!(true, "test_constexpr_forward()");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
