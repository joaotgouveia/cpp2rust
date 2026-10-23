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
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int___const_std_ranges_empty_view_int__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int__ref__const_std_ranges_empty_view_int__ref_
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_int___const_std_ranges_empty_view_const_int__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_int__ref__const_std_ranges_empty_view_const_int__ref_
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int_ptr___const_std_ranges_empty_view_int_ptr__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int_ptr__ref__const_std_ranges_empty_view_int_ptr__ref_
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_Empty___const_std_ranges_empty_view_Empty__
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_Empty__ref__const_std_ranges_empty_view_Empty__ref_
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_Empty___const_std_ranges_empty_view_const_Empty__
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_Empty__ref__const_std_ranges_empty_view_const_Empty__ref_
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_BigType___const_std_ranges_empty_view_BigType__
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_BigType__ref__const_std_ranges_empty_view_BigType__ref_
{}
pub fn testType_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let v: Value<()> = Rc::new(RefCell::new(
        (*empty_13.with(Value::clone).borrow()).clone(),
    ));
    unimplemented!();
}
pub fn testType_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let v: Value<()> = Rc::new(RefCell::new(
        (*empty_15.with(Value::clone).borrow()).clone(),
    ));
    unimplemented!();
}
pub fn testType_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let v: Value<()> = Rc::new(RefCell::new(
        (*empty_17.with(Value::clone).borrow()).clone(),
    ));
    unimplemented!();
}
pub fn testType_18() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let v: Value<()> = Rc::new(RefCell::new(
        (*empty_19.with(Value::clone).borrow()).clone(),
    ));
    unimplemented!();
}
pub fn testType_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let v: Value<()> = Rc::new(RefCell::new(
        (*empty_21.with(Value::clone).borrow()).clone(),
    ));
    unimplemented!();
}
pub fn testType_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let v: Value<()> = Rc::new(RefCell::new(
        (*empty_23.with(Value::clone).borrow()).clone(),
    ));
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct BigType {
    #[offset(0)]
    #[byte_size(8)]
    pub buff: Box<[()]>,
}
impl Default for BigType {
    fn default() -> Self {
        {
            BigType {
                buff: (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
pub fn test_24() -> () {
    ({ testType_12() });
    ({ testType_14() });
    ({ testType_16() });
    ({ testType_18() });
    ({ testType_20() });
    ({ testType_22() });
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
    ({ test_24() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
