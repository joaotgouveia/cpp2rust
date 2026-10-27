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
pub struct SizedSentinel {}
const _: () = assert!(true, "std::sized_sentinel_for<SizedSentinel, Iterator>");
const _: () = assert!(
    !(false),
    "!std::sized_sentinel_for<UnsizedSentinel, Iterator>"
);
const _: () = assert!(true, "std::ranges::sized_range<ThreeElementRange>");
const _: () = assert!(true, "std::tuple_size<SizedRange>::value == 2");
const _: () = assert!(true, "std::tuple_size<UnsizedRange>::value == 2");
const _: () = assert!(true, "std::tuple_size<ThreeElementRange>::value == 2");
pub fn test_tuple_element_0() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
}
pub fn test_tuple_element_1() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
}
pub fn test_tuple_element_2() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
}
pub fn test_tuple_element_3() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
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
    const _: () = assert!(true, "test_tuple_element<0, SizedRange, Iterator>()");;
    const _: () = assert!(true, "test_tuple_element<1, SizedRange, SizedSentinel>()");;
    const _: () = assert!(true, "test_tuple_element<0, UnsizedRange, Iterator>()");;
    const _: () = assert!(
        true,
        "test_tuple_element<1, UnsizedRange, UnsizedSentinel>()"
    );;
    return 0_();
}
pub trait SizedSentinelImpl {
    fn operator_eq(&self, mut _a0: Ptr<()>) -> () {
        unimplemented!()
    }
}
impl SizedSentinelImpl for Ptr<SizedSentinel> {}
pub fn __cpp2rust_init_globals() {}
