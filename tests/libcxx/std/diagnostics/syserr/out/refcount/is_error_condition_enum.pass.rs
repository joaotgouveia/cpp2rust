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
pub struct test_macros_detail_is_same_const_bool__const_bool_ {}
pub fn test_1() {
    const _: () = assert!(true, "(std::is_error_condition_enum<T>::value == Expected)");;
    const _: () = assert!(
        true,
        "(std::is_error_condition_enum_v<T>        == Expected)"
    );;
    const _: () = assert!(true, "");;
}
pub fn test_2() {
    const _: () = assert!(true, "(std::is_error_condition_enum<T>::value == Expected)");;
    const _: () = assert!(
        true,
        "(std::is_error_condition_enum_v<T>        == Expected)"
    );;
    const _: () = assert!(true, "");;
}
pub fn test_3() {
    const _: () = assert!(true, "(std::is_error_condition_enum<T>::value == Expected)");;
    const _: () = assert!(
        true,
        "(std::is_error_condition_enum_v<T>        == Expected)"
    );;
    const _: () = assert!(true, "");;
}
pub fn test_4() {
    const _: () = assert!(true, "(std::is_error_condition_enum<T>::value == Expected)");;
    const _: () = assert!(
        true,
        "(std::is_error_condition_enum_v<T>        == Expected)"
    );;
    const _: () = assert!(true, "");;
}
pub fn test_5() {
    const _: () = assert!(true, "(std::is_error_condition_enum<T>::value == Expected)");;
    const _: () = assert!(
        true,
        "(std::is_error_condition_enum_v<T>        == Expected)"
    );;
    const _: () = assert!(true, "");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl A {}
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
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    return 0_();
}
pub trait AImpl {
    fn to_(&self) -> ();
}
impl AImpl for Ptr<A> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
