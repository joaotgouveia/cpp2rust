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
thread_local!(
    pub static iArr1_0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
);
thread_local!(
    pub static iArr2_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ])));
);

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
    const _: () = assert!(true, "noexcept(std::span<int>().empty())");;
    const _: () = assert!(true, "noexcept(std::span<int, 0>().empty())");;
    const _: () = assert!(true, "std::span<int>().empty()");;
    const _: () = assert!(true, "std::span<long>().empty()");;
    const _: () = assert!(true, "std::span<double>().empty()");;
    const _: () = assert!(true, "std::span<A>().empty()");;
    const _: () = assert!(true, "std::span<std::string>().empty()");;
    const _: () = assert!(true, "std::span<int, 0>().empty()");;
    const _: () = assert!(true, "std::span<long, 0>().empty()");;
    const _: () = assert!(true, "std::span<double, 0>().empty()");;
    const _: () = assert!(true, "std::span<A, 0>().empty()");;
    const _: () = assert!(true, "std::span<std::string, 0>().empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 1).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 2).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 3).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 4).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 5).empty()");;
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    let s: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    assert!((unimplemented!()));
    assert!(!(unimplemented!()));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = iArr1_0.with(|_| ());
    let _ = iArr2_1.with(|_| ());
}
