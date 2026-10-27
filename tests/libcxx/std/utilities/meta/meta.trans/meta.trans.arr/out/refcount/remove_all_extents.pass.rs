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
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_Enum__const_Enum_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int__const_int_ {}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one_: Enum = 1;
pub fn test_remove_all_extents_3() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_4() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_5() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_6() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_remove_all_extents_14() {
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
    ({ test_remove_all_extents_3() });
    ({ test_remove_all_extents_4() });
    ({ test_remove_all_extents_5() });
    ({ test_remove_all_extents_6() });
    ({ test_remove_all_extents_7() });
    ({ test_remove_all_extents_8() });
    ({ test_remove_all_extents_9() });
    ({ test_remove_all_extents_10() });
    ({ test_remove_all_extents_11() });
    ({ test_remove_all_extents_12() });
    ({ test_remove_all_extents_13() });
    ({ test_remove_all_extents_14() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
