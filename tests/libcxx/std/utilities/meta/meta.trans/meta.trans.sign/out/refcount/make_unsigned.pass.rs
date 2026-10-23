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
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_short__unsigned_short_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long_long__unsigned_long_long_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_unsigned_int__const_unsigned_int_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned___int128__unsigned___int128_ {}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one_: Enum = 1;
pub type BigEnum = ();
pub const BigEnum_bigzero: BigEnum = 0;
pub const BigEnum_big: BigEnum = 18446744073709551615;
pub type HugeEnum = ();
pub const HugeEnum_hugezero: HugeEnum = 0;
pub fn test_make_unsigned_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_17() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_18() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_19() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_21() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_23() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_make_unsigned_24() {
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
    ({ test_make_unsigned_7() });
    ({ test_make_unsigned_8() });
    ({ test_make_unsigned_9() });
    ({ test_make_unsigned_10() });
    ({ test_make_unsigned_11() });
    ({ test_make_unsigned_12() });
    ({ test_make_unsigned_13() });
    ({ test_make_unsigned_14() });
    ({ test_make_unsigned_15() });
    ({ test_make_unsigned_16() });
    ({ test_make_unsigned_17() });
    ({ test_make_unsigned_18() });
    ({ test_make_unsigned_19() });
    ({ test_make_unsigned_20() });
    ({ test_make_unsigned_21() });
    ({ test_make_unsigned_22() });
    ({ test_make_unsigned_23() });
    ({ test_make_unsigned_24() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
