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
pub struct test_macros_detail_is_same_intarr3arr__intarr3arr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub fn test_remove_cv_imp_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_17() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_18() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_19() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_21() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_23() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_24() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_25() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_26() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_27() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_imp_28() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_cv_29() {
    ({ test_remove_cv_imp_7() });
    ({ test_remove_cv_imp_8() });
    ({ test_remove_cv_imp_9() });
    ({ test_remove_cv_imp_10() });
}
pub fn test_remove_cv_30() {
    ({ test_remove_cv_imp_11() });
    ({ test_remove_cv_imp_12() });
    ({ test_remove_cv_imp_13() });
    ({ test_remove_cv_imp_14() });
}
pub fn test_remove_cv_31() {
    ({ test_remove_cv_imp_15() });
    ({ test_remove_cv_imp_16() });
    ({ test_remove_cv_imp_17() });
    ({ test_remove_cv_imp_18() });
}
pub fn test_remove_cv_32() {
    ({ test_remove_cv_imp_19() });
    ({ test_remove_cv_imp_19() });
    ({ test_remove_cv_imp_19() });
    ({ test_remove_cv_imp_19() });
}
pub fn test_remove_cv_33() {
    ({ test_remove_cv_imp_20() });
    ({ test_remove_cv_imp_20() });
    ({ test_remove_cv_imp_20() });
    ({ test_remove_cv_imp_20() });
}
pub fn test_remove_cv_34() {
    ({ test_remove_cv_imp_21() });
    ({ test_remove_cv_imp_22() });
    ({ test_remove_cv_imp_23() });
    ({ test_remove_cv_imp_24() });
}
pub fn test_remove_cv_35() {
    ({ test_remove_cv_imp_25() });
    ({ test_remove_cv_imp_26() });
    ({ test_remove_cv_imp_27() });
    ({ test_remove_cv_imp_28() });
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
    ({ test_remove_cv_29() });
    ({ test_remove_cv_30() });
    ({ test_remove_cv_31() });
    ({ test_remove_cv_32() });
    ({ test_remove_cv_33() });
    ({ test_remove_cv_34() });
    ({ test_remove_cv_35() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
