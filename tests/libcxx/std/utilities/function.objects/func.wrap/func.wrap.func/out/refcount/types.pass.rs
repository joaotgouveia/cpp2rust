extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_argument_type_std_function_int_____ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_argument_type_std_function_double__int__char___ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_argument_type_std_function_double__int__char__double___ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_first_argument_type_std_function_int_____ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_first_argument_type_std_function_double__int___ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_first_argument_type_std_function_double__int__char__double___ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_second_argument_type_std_function_int_____ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_second_argument_type_std_function_double__int___ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct has_second_argument_type_std_function_double__int__char__double___ {}
pub fn test_nullary_function_9() {
    const _: () = assert!(true, "(!has_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_first_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_second_argument_type<F>::value)");;
}
pub fn test_unary_function_10() {
    const _: () = assert!(true, "(!has_first_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_second_argument_type<F>::value)");;
}
pub fn test_binary_function_11() {
    const _: () = assert!(true, "(!has_argument_type<F>::value)");;
}
pub fn test_other_function_12() {
    const _: () = assert!(true, "(!has_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_first_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_second_argument_type<F>::value)");;
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
    ({ test_nullary_function_9() });
    ({ test_unary_function_10() });
    ({ test_binary_function_11() });
    ({ test_other_function_12() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
