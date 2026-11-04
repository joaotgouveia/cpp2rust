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
impl CppMove for Other_1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Other_3 {}
impl CppMove for Other_3 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_ctor_4() -> bool {
    let val: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
    let s1: Value<std_span_int__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_int__18446744073709551615_::new_1({ (val.as_pointer() as Ptr<i32>) }, { 2_u64 }),
    ));
    let s2: Value<std_span_int__2_> = Rc::new(RefCell::new(std_span_int__2_::new_1(
        { (val.as_pointer() as Ptr<i32>) },
        { 2_u64 },
    )));
    assert!(
        ({
            (({ (*s1.borrow()).data() }) == ({ data_5(val.as_pointer()) }))
                && ((({ (*s1.borrow()).size() }) as usize) == ({ size_6(val.as_pointer()) }))
        })
    );
    assert!(
        ({
            (({ (*s2.borrow()).data() }) == ({ data_5(val.as_pointer()) }))
                && ((({ (*s2.borrow()).size() }) as usize) == ({ size_6(val.as_pointer()) }))
        })
    );
    return true;
}
pub fn test_ctor_7() -> bool {
    let val: Value<Box<[A_8]>> =
        Rc::new(RefCell::new(Box::new([<A_8>::default(), <A_8>::default()])));
    let s1: Value<std_span_A__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_A__18446744073709551615_::new_1({ (val.as_pointer() as Ptr<A_8>) }, { 2_u64 }),
    ));
    let s2: Value<std_span_A__2_> = Rc::new(RefCell::new(std_span_A__2_::new_1(
        { (val.as_pointer() as Ptr<A_8>) },
        { 2_u64 },
    )));
    assert!(
        ({
            (({ (*s1.borrow()).data() }) == ({ data_9(val.as_pointer()) }))
                && ((({ (*s1.borrow()).size() }) as usize) == ({ size_10(val.as_pointer()) }))
        })
    );
    assert!(
        ({
            (({ (*s2.borrow()).data() }) == ({ data_9(val.as_pointer()) }))
                && ((({ (*s2.borrow()).size() }) as usize) == ({ size_10(val.as_pointer()) }))
        })
    );
    return true;
}
pub fn test_11() -> bool {
    ({ test_constructibility_0() });
    ({ test_constructibility_2() });
    ({ test_ctor_4() });
    ({ test_ctor_7() });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_8 {}
impl CppMove for A_8 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_11() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
