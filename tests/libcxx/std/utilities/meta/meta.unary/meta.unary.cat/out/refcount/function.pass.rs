extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
pub type Enum1 = ();
pub type Enum2 = ();
pub fn test_0() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_1() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_2() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_3() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_4() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_5() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_6() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_7() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_8() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_9() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_10() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_11() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_12() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_13() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_14() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_15() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_16() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_17() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_18() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_19() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_20() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_21() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_22() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_23() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_24() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_25() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_26() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_27() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_28() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_29() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_30() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_31() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_32() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_33() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_34() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_35() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_36() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_37() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_38() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_39() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_40() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_41() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_42() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_43() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_44() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_45() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_46() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_47() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_48() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_49() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_50() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_51() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_52() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_53() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_54() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_55() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_56() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_57() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_58() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_59() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_60() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_61() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_62() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_63() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_64() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_65() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_66() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_67() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_68() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_69() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_70() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_71() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_72() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_73() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_74() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_75() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_76() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_77() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_78() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_79() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_80() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_81() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_82() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_83() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_84() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_85() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_86() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_87() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_88() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_89() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_90() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_91() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_92() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_93() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_94() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
}
pub fn test_95() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "std::is_function<T>::value");;
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    ({ test_9() });
    ({ test_10() });
    ({ test_11() });
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    ({ test_18() });
    ({ test_19() });
    ({ test_20() });
    ({ test_21() });
    ({ test_22() });
    ({ test_23() });
    ({ test_24() });
    ({ test_25() });
    ({ test_26() });
    ({ test_27() });
    ({ test_28() });
    ({ test_29() });
    ({ test_30() });
    ({ test_31() });
    ({ test_32() });
    ({ test_33() });
    ({ test_34() });
    ({ test_35() });
    ({ test_36() });
    ({ test_37() });
    ({ test_38() });
    ({ test_39() });
    ({ test_40() });
    ({ test_41() });
    ({ test_42() });
    ({ test_43() });
    ({ test_44() });
    ({ test_45() });
    ({ test_46() });
    ({ test_47() });
    ({ test_48() });
    ({ test_49() });
    ({ test_50() });
    ({ test_51() });
    ({ test_52() });
    ({ test_53() });
    ({ test_54() });
    ({ test_55() });
    ({ test_56() });
    ({ test_57() });
    ({ test_58() });
    ({ test_59() });
    ({ test_60() });
    ({ test_61() });
    ({ test_62() });
    ({ test_63() });
    ({ test_64() });
    ({ test_65() });
    ({ test_66() });
    ({ test_67() });
    ({ test_68() });
    ({ test_69() });
    ({ test_70() });
    ({ test_71() });
    ({ test_72() });
    ({ test_73() });
    ({ test_74() });
    ({ test_75() });
    ({ test_76() });
    ({ test_77() });
    ({ test_78() });
    ({ test_79() });
    ({ test_80() });
    ({ test_81() });
    ({ test_82() });
    ({ test_83() });
    ({ test_84() });
    ({ test_85() });
    ({ test_86() });
    ({ test_87() });
    ({ test_88() });
    ({ test_89() });
    ({ test_90() });
    ({ test_91() });
    ({ test_92() });
    ({ test_93() });
    ({ test_94() });
    ({ test_95() });
    const _: () = assert!(true, "!std::is_function<incomplete_type>::value");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
