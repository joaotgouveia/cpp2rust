extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_complete_0(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_1(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_2(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_3(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_4(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_5(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_6(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_7(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_8(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_9(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_10(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_11(mut _a0: i32) -> bool {
    const _: () = assert!((1_u64 > 0_u64), "Size > 0");;
    return true;
}
pub fn is_complete_12(mut _a0: i64) -> bool {
    return false;
}
pub fn is_complete_13(mut _a0: i64) -> bool {
    return false;
}
pub fn is_complete_14(mut _a0: i64) -> bool {
    return false;
}
pub fn is_complete_15() -> bool {
    return ({ is_complete_0(0) });
}
pub fn is_complete_16() -> bool {
    return ({ is_complete_1(0) });
}
pub fn is_complete_17() -> bool {
    return ({ is_complete_2(0) });
}
pub fn is_complete_18() -> bool {
    return ({ is_complete_3(0) });
}
pub fn is_complete_19() -> bool {
    return ({ is_complete_4(0) });
}
pub fn is_complete_20() -> bool {
    return ({ is_complete_5(0) });
}
pub fn is_complete_21() -> bool {
    return ({ is_complete_6(0) });
}
pub fn is_complete_22() -> bool {
    return ({ is_complete_7(0) });
}
pub fn is_complete_23() -> bool {
    return ({ is_complete_8(0) });
}
pub fn is_complete_24() -> bool {
    return ({ is_complete_9(0) });
}
pub fn is_complete_25() -> bool {
    return ({ is_complete_10(0) });
}
pub fn is_complete_26() -> bool {
    return ({ is_complete_11(0) });
}
pub fn is_complete_27() -> bool {
    return ({ is_complete_12(0_i64) });
}
pub fn is_complete_28() -> bool {
    return ({ is_complete_13(0_i64) });
}
pub fn is_complete_29() -> bool {
    return ({ is_complete_14(0_i64) });
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
pub fn test_complete_30() {
    const _: () = assert!(true, "is_complete<T>()");;
    const _: () = assert!(true, "is_complete<const T>()");;
    const _: () = assert!(true, "is_complete<volatile T>()");;
    const _: () = assert!(true, "is_complete<const volatile T>()");;
}
pub fn test_complete_31() {
    const _: () = assert!(true, "is_complete<T>()");;
    const _: () = assert!(true, "is_complete<const T>()");;
    const _: () = assert!(true, "is_complete<volatile T>()");;
    const _: () = assert!(true, "is_complete<const volatile T>()");;
}
pub fn test_complete_32() {
    const _: () = assert!(true, "is_complete<T>()");;
    const _: () = assert!(true, "is_complete<const T>()");;
    const _: () = assert!(true, "is_complete<volatile T>()");;
    const _: () = assert!(true, "is_complete<const volatile T>()");;
}
pub fn test_incomplete_33() {
    const _: () = assert!(true, "!is_complete<T>()");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_incomplete_34() {
    const _: () = assert!(true, "!is_complete<T>()");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_incomplete_35() {
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
    ({ test_complete_36() });
    ({ test_complete_37() });
    ({ test_complete_38() });
    ({ test_complete_30() });
    ({ test_complete_31() });
    ({ test_complete_32() });
    ({ test_incomplete_33() });
    ({ test_incomplete_34() });
    ({ test_incomplete_39() });
    ({ test_incomplete_35() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
