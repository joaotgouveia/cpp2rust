extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_float_ptr__float_ptr_ {}
impl CppMove for test_macros_detail_is_same_float_ptr__float_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_float_ptr__const_float_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_float_ptr__const_float_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
impl CppMove for test_macros_detail_is_same_double_ptr__double_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_double_ptr__const_double_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_double_ptr__const_double_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_X_ptr__X_ptr_ {}
impl CppMove for test_macros_detail_is_same_X_ptr__X_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_X_ptr__const_X_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_X_ptr__const_X_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X {}
impl CppMove for X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_contract_8() {
    let opt: Value<std_optional_int_> = Rc::new(RefCell::new(<std_optional_int_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_9() {
    let opt: Value<std_optional_float_> = Rc::new(RefCell::new(<std_optional_float_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_10() {
    let opt: Value<std_optional_double_> = Rc::new(RefCell::new(<std_optional_double_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_11() {
    let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<std_optional_X_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_12() {
    let opt: Value<std_optional_const_int_> =
        Rc::new(RefCell::new(<std_optional_const_int_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_13() {
    let opt: Value<std_optional_const_float_> =
        Rc::new(RefCell::new(<std_optional_const_float_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_14() {
    let opt: Value<std_optional_const_double_> =
        Rc::new(RefCell::new(<std_optional_const_double_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_contract_15() {
    let opt: Value<std_optional_const_X_> =
        Rc::new(RefCell::new(<std_optional_const_X_>::default()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_16() -> bool {
    ({ test_contract_8() });
    ({ test_contract_9() });
    ({ test_contract_10() });
    ({ test_contract_11() });
    ({ test_contract_12() });
    ({ test_contract_13() });
    ({ test_contract_14() });
    ({ test_contract_15() });
    let opt: Value<std_optional_X_> = Rc::new(RefCell::new(<X>::default()));
    {
        ({ cpp2rust_assert_fail_17((({ XImpl::test_1(&opt.as_pointer()) }) == 3)) });
        ({ cpp2rust_assert_fail_17((({ XImpl::test_2(&opt.as_pointer()) }) == 4)) });
        ({
            cpp2rust_assert_fail_17(
                (({ (*opt.borrow()).operator_arrow_16() }) == (opt.as_pointer())),
            )
        });
        ({
            cpp2rust_assert_fail_17(
                (({ (*opt.borrow()).operator_arrow_15() }) == (opt.as_pointer())),
            )
        });
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ cpp2rust_assert_fail_17(({ test_16() })) });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait XImpl {
    fn test_1(&self) -> i32;
    fn test_2(&self) -> i32;
}
impl XImpl for Ptr<X> {
    fn test_1(&self) -> i32 {
        return 3;
    }
    fn test_2(&self) -> i32 {
        return 4;
    }
}
pub fn __cpp2rust_init_globals() {}
