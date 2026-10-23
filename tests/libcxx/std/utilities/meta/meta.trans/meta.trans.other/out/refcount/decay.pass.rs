extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_void__void_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_void__ptr_____void__ptr____ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int__const__int__int__const_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int__volatile__int__int__volatile_ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int__ref__int__int__ref_ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int__refref__int__int__refref_ {}
pub fn test_decay_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_17() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_18() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_19() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_21() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_decay_23() {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_decay_9() });
    ({ test_decay_10() });
    ({ test_decay_11() });
    ({ test_decay_12() });
    ({ test_decay_13() });
    ({ test_decay_14() });
    ({ test_decay_15() });
    ({ test_decay_16() });
    ({ test_decay_17() });
    ({ test_decay_18() });
    ({ test_decay_19() });
    ({ test_decay_20() });
    ({ test_decay_21() });
    ({ test_decay_22() });
    ({ test_decay_23() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
