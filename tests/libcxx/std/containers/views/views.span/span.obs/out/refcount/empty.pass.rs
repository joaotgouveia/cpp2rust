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
    pub static iArr1_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
    ])));
);
thread_local!(
    pub static iArr2_1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
        16_(),
        17_(),
        18_(),
        19_(),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = iArr1_0.with(|_| ());
    let _ = iArr2_1.with(|_| ());
}
