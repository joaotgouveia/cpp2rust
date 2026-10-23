extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Class {}
pub type Enum1 = ();
pub type Enum2 = ();
pub unsafe fn test_0() {
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
pub unsafe fn test_1() {
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
pub unsafe fn test_2() {
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
pub unsafe fn test_3() {
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
pub unsafe fn test_4() {
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
pub unsafe fn test_5() {
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
pub unsafe fn test_6() {
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
pub unsafe fn test_7() {
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
pub unsafe fn test_8() {
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
pub unsafe fn test_9() {
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
pub unsafe fn test_10() {
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
pub unsafe fn test_11() {
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
pub unsafe fn test_12() {
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
pub unsafe fn test_13() {
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
pub unsafe fn test_14() {
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
pub unsafe fn test_15() {
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
pub unsafe fn test_16() {
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
pub unsafe fn test_17() {
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
pub unsafe fn test_18() {
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
pub unsafe fn test_19() {
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
pub unsafe fn test_20() {
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
pub unsafe fn test_21() {
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
pub unsafe fn test_22() {
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
pub unsafe fn test_23() {
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
pub unsafe fn test_24() {
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
pub unsafe fn test_25() {
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
pub unsafe fn test_26() {
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
pub unsafe fn test_27() {
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
pub unsafe fn test_28() {
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
pub unsafe fn test_29() {
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
pub unsafe fn test_30() {
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
pub unsafe fn test_31() {
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
pub unsafe fn test_32() {
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
pub unsafe fn test_33() {
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
pub unsafe fn test_34() {
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
pub unsafe fn test_35() {
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
pub unsafe fn test_36() {
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
pub unsafe fn test_37() {
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
pub unsafe fn test_38() {
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
pub unsafe fn test_39() {
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
pub unsafe fn test_40() {
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
pub unsafe fn test_41() {
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
pub unsafe fn test_42() {
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
pub unsafe fn test_43() {
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
pub unsafe fn test_44() {
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
pub unsafe fn test_45() {
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
pub unsafe fn test_46() {
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
pub unsafe fn test_47() {
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
pub unsafe fn test_48() {
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
pub unsafe fn test_49() {
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
pub unsafe fn test_50() {
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
pub unsafe fn test_51() {
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
pub unsafe fn test_52() {
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
pub unsafe fn test_53() {
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
pub unsafe fn test_54() {
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
pub unsafe fn test_55() {
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
pub unsafe fn test_56() {
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
pub unsafe fn test_57() {
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
pub unsafe fn test_58() {
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
pub unsafe fn test_59() {
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
pub unsafe fn test_60() {
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
pub unsafe fn test_61() {
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
pub unsafe fn test_62() {
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
pub unsafe fn test_63() {
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
pub unsafe fn test_64() {
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
pub unsafe fn test_65() {
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
pub unsafe fn test_66() {
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
pub unsafe fn test_67() {
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
pub unsafe fn test_68() {
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
pub unsafe fn test_69() {
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
pub unsafe fn test_70() {
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
pub unsafe fn test_71() {
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
pub unsafe fn test_72() {
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
pub unsafe fn test_73() {
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
pub unsafe fn test_74() {
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
pub unsafe fn test_75() {
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
pub unsafe fn test_76() {
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
pub unsafe fn test_77() {
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
pub unsafe fn test_78() {
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
pub unsafe fn test_79() {
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
pub unsafe fn test_80() {
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
pub unsafe fn test_81() {
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
pub unsafe fn test_82() {
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
pub unsafe fn test_83() {
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
pub unsafe fn test_84() {
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
pub unsafe fn test_85() {
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
pub unsafe fn test_86() {
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
pub unsafe fn test_87() {
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
pub unsafe fn test_88() {
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
pub unsafe fn test_89() {
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
pub unsafe fn test_90() {
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
pub unsafe fn test_91() {
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
pub unsafe fn test_92() {
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
pub unsafe fn test_93() {
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
pub unsafe fn test_94() {
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
pub unsafe fn test_95() {
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
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_9() });
    (unsafe { test_10() });
    (unsafe { test_11() });
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_20() });
    (unsafe { test_21() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    (unsafe { test_31() });
    (unsafe { test_32() });
    (unsafe { test_33() });
    (unsafe { test_34() });
    (unsafe { test_35() });
    (unsafe { test_36() });
    (unsafe { test_37() });
    (unsafe { test_38() });
    (unsafe { test_39() });
    (unsafe { test_40() });
    (unsafe { test_41() });
    (unsafe { test_42() });
    (unsafe { test_43() });
    (unsafe { test_44() });
    (unsafe { test_45() });
    (unsafe { test_46() });
    (unsafe { test_47() });
    (unsafe { test_48() });
    (unsafe { test_49() });
    (unsafe { test_50() });
    (unsafe { test_51() });
    (unsafe { test_52() });
    (unsafe { test_53() });
    (unsafe { test_54() });
    (unsafe { test_55() });
    (unsafe { test_56() });
    (unsafe { test_57() });
    (unsafe { test_58() });
    (unsafe { test_59() });
    (unsafe { test_60() });
    (unsafe { test_61() });
    (unsafe { test_62() });
    (unsafe { test_63() });
    (unsafe { test_64() });
    (unsafe { test_65() });
    (unsafe { test_66() });
    (unsafe { test_67() });
    (unsafe { test_68() });
    (unsafe { test_69() });
    (unsafe { test_70() });
    (unsafe { test_71() });
    (unsafe { test_72() });
    (unsafe { test_73() });
    (unsafe { test_74() });
    (unsafe { test_75() });
    (unsafe { test_76() });
    (unsafe { test_77() });
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    (unsafe { test_81() });
    (unsafe { test_82() });
    (unsafe { test_83() });
    (unsafe { test_84() });
    (unsafe { test_85() });
    (unsafe { test_86() });
    (unsafe { test_87() });
    (unsafe { test_88() });
    (unsafe { test_89() });
    (unsafe { test_90() });
    (unsafe { test_91() });
    (unsafe { test_92() });
    (unsafe { test_93() });
    (unsafe { test_94() });
    (unsafe { test_95() });
    const _: () = assert!(true, "!std::is_function<incomplete_type>::value");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
