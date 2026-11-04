extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_complete_0(mut _a0: i32) -> bool {
    const _: () = assert!(true, "Size > 0");;
    return true;
}
pub fn is_complete_1(mut _a0: i32) -> bool {
    const _: () = assert!(true, "Size > 0");;
    return true;
}
pub fn is_complete_2(mut _a0: i32) -> bool {
    const _: () = assert!(true, "Size > 0");;
    return true;
}
pub fn is_complete_3(mut _a0: i32) -> bool {
    const _: () = assert!(true, "Size > 0");;
    return true;
}
pub fn is_complete_4(mut _a0: i64) -> bool {
    return false;
}
pub fn is_complete_5(mut _a0: i64) -> bool {
    return false;
}
pub fn is_complete_6(mut _a0: i64) -> bool {
    return false;
}
pub fn is_complete_7() -> bool {
    return ({ is_complete_0(0) });
}
pub fn is_complete_8() -> bool {
    return ({ is_complete_1(0) });
}
pub fn is_complete_9() -> bool {
    return ({ is_complete_2(0) });
}
pub fn is_complete_10() -> bool {
    return ({ is_complete_3(0) });
}
pub fn is_complete_11() -> bool {
    return ({ is_complete_4(0_i64) });
}
pub fn is_complete_12() -> bool {
    return ({ is_complete_5(0_i64) });
}
pub fn is_complete_13() -> bool {
    return ({ is_complete_6(0_i64) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Dummy1 {}
impl CppMove for Dummy1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Dummy2 {}
impl CppMove for Dummy2 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_complete_14() {
    const _: () = assert!(true, "is_complete<T>()");;
    const _: () = assert!(true, "is_complete<const T>()");;
    const _: () = assert!(true, "is_complete<volatile T>()");;
    const _: () = assert!(true, "is_complete<const volatile T>()");;
}
pub fn test_incomplete_15() {
    const _: () = assert!(true, "!is_complete<T>()");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_incomplete_16() {
    const _: () = assert!(true, "!is_complete<T>()");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_incomplete_17() {
    const _: () = assert!(true, "!is_complete<T>()");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    ({ test_complete_18() });
    ({ test_complete_19() });
    ({ test_complete_20() });
    ({ test_complete_21() });
    ({ test_complete_22() });
    ({ test_complete_14() });
    ({ test_incomplete_15() });
    ({ test_incomplete_16() });
    ({ test_incomplete_23() });
    ({ test_incomplete_17() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
