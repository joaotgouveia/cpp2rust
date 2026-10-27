extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constructibility_0() {
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, double*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, std::size_t, size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, Other*, std::size_t>"
    );;
}
pub fn test_constructibility_2() {
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<std::span<const int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<const int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<volatile int, Extent>, const volatile int*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, double*, std::size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, std::size_t, size_t>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::span<int, Extent>, Other*, std::size_t>"
    );;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Other_1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Other_3 {}
pub fn test_ctor_4() -> () {
    let val: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([unimplemented!(), unimplemented!()])));
    let s1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let s2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_ctor_5() -> () {
    let val: Value<Box<[A_6]>> =
        Rc::new(RefCell::new(Box::new([<A_6>::default(), <A_6>::default()])));
    let s1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let s2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_7() -> () {
    ({ test_constructibility_0() });
    ({ test_constructibility_2() });
    ({ test_ctor_4() });
    ({ test_ctor_5() });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_6 {}
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
