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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_volatile_char_ptr__volatile_char_ptr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_volatile_char_ptr__const_volatile_char_ptr_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_volatile_int_ptr__volatile_int_ptr_ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_volatile_int_ptr__const_volatile_int_ptr_ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_double_ptr__const_double_ptr_ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_volatile_double_ptr__volatile_double_ptr_ {}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_volatile_double_ptr__const_volatile_double_ptr_ {}
const _: () = assert!(!(false), "!has_midpoint<std::nullptr_t>");
const _: () = assert!(!(false), "!has_midpoint<FuncPtr>");
const _: () = assert!(!(false), "!has_midpoint<Incomplete*>");
const _: () = assert!(!(false), "!has_midpoint<void*>");
const _: () = assert!(!(false), "!has_midpoint<const void*>");
const _: () = assert!(!(false), "!has_midpoint<volatile void*>");
const _: () = assert!(!(false), "!has_midpoint<const volatile void*>");
pub fn check_12(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_13(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_14(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_15(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_16(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_17(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_18(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_19(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_20(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_21(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_22(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_23(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_24(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_25(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_26(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_27(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_28(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_29(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_30(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_31(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_32(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_33(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn check_34(mut base: Ptr<()>, mut i: (), mut j: (), mut expect: ()) -> () {
    return ({
        ({
            let ___a: Ptr<()> = base.offset((i) as isize);
            let ___b: Ptr<()> = base.offset((j) as isize);
            midpoint_35(___a, ___b)
        })
    } == { base.offset((expect) as isize) });
}
pub fn test_pointer_36() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_37() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_38() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_39() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_40() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_41() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_42() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_43() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_44() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_45() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_46() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_pointer_47() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let array: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
        <()>::default(),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_48() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
pub fn test_49() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
}
pub fn test_50() {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test_pointer<T>()");;
    const _: () = assert!(true, "test_pointer<const T>()");;
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
    ({ test_48() });
    ({ test_49() });
    ({ test_50() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
