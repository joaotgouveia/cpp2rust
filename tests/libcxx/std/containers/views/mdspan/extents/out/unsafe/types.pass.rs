extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_long__long_long_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long_long__unsigned_long_long_ {}
pub unsafe fn testExtents_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_17() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_18() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_19() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_21() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_23() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_24() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_25() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_26() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_27() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_28() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_29() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_30() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_31() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_32() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_33() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_34() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_35() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_36() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_37() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_38() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_39() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_40() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_41() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_42() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_43() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_44() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_45() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_46() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_47() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_48() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_49() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_50() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_51() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_52() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_53() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_54() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_55() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_56() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_57() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_58() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_59() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_60() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_61() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_62() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_63() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_64() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_65() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_66() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_67() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_68() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_69() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_70() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_71() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_72() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_73() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_74() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_75() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_76() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_77() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_78() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_79() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_80() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_81() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_82() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_83() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_84() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_85() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_86() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_87() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_88() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_89() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_90() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_91() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "sizeof...(Extents) == E::rank()");;
    const _: () = assert!(
        true,
        "static_cast<size_t>(Extents == std::dynamic_extent) + ...) == E::rank_dynamic()"
    );;
    const _: () = assert!(true, "std::regular<E>");;
    const _: () = assert!(true, "std::is_trivially_copyable_v<E>");;
    const _: () = assert!(true, "std::is_empty_v<E> == (E::rank_dynamic() == 0)");;
}
pub unsafe fn testExtents_92() {
    (unsafe { testExtents_7() });
}
pub unsafe fn testExtents_93() {
    (unsafe { testExtents_8() });
}
pub unsafe fn testExtents_94() {
    (unsafe { testExtents_9() });
}
pub unsafe fn testExtents_95() {
    (unsafe { testExtents_10() });
}
pub unsafe fn testExtents_96() {
    (unsafe { testExtents_11() });
}
pub unsafe fn testExtents_97() {
    (unsafe { testExtents_12() });
}
pub unsafe fn testExtents_98() {
    (unsafe { testExtents_13() });
}
pub unsafe fn testExtents_99() {
    (unsafe { testExtents_14() });
}
pub unsafe fn testExtents_100() {
    (unsafe { testExtents_15() });
}
pub unsafe fn testExtents_101() {
    (unsafe { testExtents_16() });
}
pub unsafe fn testExtents_102() {
    (unsafe { testExtents_17() });
}
pub unsafe fn testExtents_103() {
    (unsafe { testExtents_18() });
}
pub unsafe fn testExtents_104() {
    (unsafe { testExtents_19() });
}
pub unsafe fn testExtents_105() {
    (unsafe { testExtents_20() });
}
pub unsafe fn testExtents_106() {
    (unsafe { testExtents_21() });
}
pub unsafe fn testExtents_107() {
    (unsafe { testExtents_22() });
}
pub unsafe fn testExtents_108() {
    (unsafe { testExtents_23() });
}
pub unsafe fn testExtents_109() {
    (unsafe { testExtents_24() });
}
pub unsafe fn testExtents_110() {
    (unsafe { testExtents_25() });
}
pub unsafe fn testExtents_111() {
    (unsafe { testExtents_26() });
}
pub unsafe fn testExtents_112() {
    (unsafe { testExtents_27() });
}
pub unsafe fn testExtents_113() {
    (unsafe { testExtents_28() });
}
pub unsafe fn testExtents_114() {
    (unsafe { testExtents_29() });
}
pub unsafe fn testExtents_115() {
    (unsafe { testExtents_30() });
}
pub unsafe fn testExtents_116() {
    (unsafe { testExtents_31() });
}
pub unsafe fn testExtents_117() {
    (unsafe { testExtents_32() });
}
pub unsafe fn testExtents_118() {
    (unsafe { testExtents_33() });
}
pub unsafe fn testExtents_119() {
    (unsafe { testExtents_34() });
}
pub unsafe fn testExtents_120() {
    (unsafe { testExtents_35() });
}
pub unsafe fn testExtents_121() {
    (unsafe { testExtents_36() });
}
pub unsafe fn testExtents_122() {
    (unsafe { testExtents_37() });
}
pub unsafe fn testExtents_123() {
    (unsafe { testExtents_38() });
}
pub unsafe fn testExtents_124() {
    (unsafe { testExtents_39() });
}
pub unsafe fn testExtents_125() {
    (unsafe { testExtents_40() });
}
pub unsafe fn testExtents_126() {
    (unsafe { testExtents_41() });
}
pub unsafe fn testExtents_127() {
    (unsafe { testExtents_42() });
}
pub unsafe fn testExtents_128() {
    (unsafe { testExtents_43() });
}
pub unsafe fn testExtents_129() {
    (unsafe { testExtents_44() });
}
pub unsafe fn testExtents_130() {
    (unsafe { testExtents_45() });
}
pub unsafe fn testExtents_131() {
    (unsafe { testExtents_46() });
}
pub unsafe fn testExtents_132() {
    (unsafe { testExtents_47() });
}
pub unsafe fn testExtents_133() {
    (unsafe { testExtents_48() });
}
pub unsafe fn testExtents_134() {
    (unsafe { testExtents_49() });
}
pub unsafe fn testExtents_135() {
    (unsafe { testExtents_50() });
}
pub unsafe fn testExtents_136() {
    (unsafe { testExtents_51() });
}
pub unsafe fn testExtents_137() {
    (unsafe { testExtents_52() });
}
pub unsafe fn testExtents_138() {
    (unsafe { testExtents_53() });
}
pub unsafe fn testExtents_139() {
    (unsafe { testExtents_54() });
}
pub unsafe fn testExtents_140() {
    (unsafe { testExtents_55() });
}
pub unsafe fn testExtents_141() {
    (unsafe { testExtents_56() });
}
pub unsafe fn testExtents_142() {
    (unsafe { testExtents_57() });
}
pub unsafe fn testExtents_143() {
    (unsafe { testExtents_58() });
}
pub unsafe fn testExtents_144() {
    (unsafe { testExtents_59() });
}
pub unsafe fn testExtents_145() {
    (unsafe { testExtents_60() });
}
pub unsafe fn testExtents_146() {
    (unsafe { testExtents_61() });
}
pub unsafe fn testExtents_147() {
    (unsafe { testExtents_62() });
}
pub unsafe fn testExtents_148() {
    (unsafe { testExtents_63() });
}
pub unsafe fn testExtents_149() {
    (unsafe { testExtents_64() });
}
pub unsafe fn testExtents_150() {
    (unsafe { testExtents_65() });
}
pub unsafe fn testExtents_151() {
    (unsafe { testExtents_66() });
}
pub unsafe fn testExtents_152() {
    (unsafe { testExtents_67() });
}
pub unsafe fn testExtents_153() {
    (unsafe { testExtents_68() });
}
pub unsafe fn testExtents_154() {
    (unsafe { testExtents_69() });
}
pub unsafe fn testExtents_155() {
    (unsafe { testExtents_70() });
}
pub unsafe fn testExtents_156() {
    (unsafe { testExtents_71() });
}
pub unsafe fn testExtents_157() {
    (unsafe { testExtents_72() });
}
pub unsafe fn testExtents_158() {
    (unsafe { testExtents_73() });
}
pub unsafe fn testExtents_159() {
    (unsafe { testExtents_74() });
}
pub unsafe fn testExtents_160() {
    (unsafe { testExtents_75() });
}
pub unsafe fn testExtents_161() {
    (unsafe { testExtents_76() });
}
pub unsafe fn testExtents_162() {
    (unsafe { testExtents_77() });
}
pub unsafe fn testExtents_163() {
    (unsafe { testExtents_78() });
}
pub unsafe fn testExtents_164() {
    (unsafe { testExtents_79() });
}
pub unsafe fn testExtents_165() {
    (unsafe { testExtents_80() });
}
pub unsafe fn testExtents_166() {
    (unsafe { testExtents_81() });
}
pub unsafe fn testExtents_167() {
    (unsafe { testExtents_82() });
}
pub unsafe fn testExtents_168() {
    (unsafe { testExtents_83() });
}
pub unsafe fn testExtents_169() {
    (unsafe { testExtents_84() });
}
pub unsafe fn testExtents_170() {
    (unsafe { testExtents_85() });
}
pub unsafe fn testExtents_171() {
    (unsafe { testExtents_86() });
}
pub unsafe fn testExtents_172() {
    (unsafe { testExtents_87() });
}
pub unsafe fn testExtents_173() {
    (unsafe { testExtents_88() });
}
pub unsafe fn testExtents_174() {
    (unsafe { testExtents_89() });
}
pub unsafe fn testExtents_175() {
    (unsafe { testExtents_90() });
}
pub unsafe fn testExtents_176() {
    (unsafe { testExtents_91() });
}
pub unsafe fn test_177() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_178));
    (unsafe { testExtents_92() });
    (unsafe { testExtents_93() });
    (unsafe { testExtents_94() });
    (unsafe { testExtents_95() });
    (unsafe { testExtents_96() });
    (unsafe { testExtents_97() });
    (unsafe { testExtents_98() });
    (unsafe { testExtents_99() });
    (unsafe { testExtents_100() });
    (unsafe { testExtents_101() });
    (unsafe { testExtents_102() });
    (unsafe { testExtents_103() });
    (unsafe { testExtents_104() });
    (unsafe { testExtents_105() });
    (unsafe { testExtents_106() });
    (unsafe { testExtents_107() });
    (unsafe { testExtents_108() });
}
pub unsafe fn test_179() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_178));
    (unsafe { testExtents_109() });
    (unsafe { testExtents_110() });
    (unsafe { testExtents_111() });
    (unsafe { testExtents_112() });
    (unsafe { testExtents_113() });
    (unsafe { testExtents_114() });
    (unsafe { testExtents_115() });
    (unsafe { testExtents_116() });
    (unsafe { testExtents_117() });
    (unsafe { testExtents_118() });
    (unsafe { testExtents_119() });
    (unsafe { testExtents_120() });
    (unsafe { testExtents_121() });
    (unsafe { testExtents_122() });
    (unsafe { testExtents_123() });
    (unsafe { testExtents_124() });
    (unsafe { testExtents_125() });
}
pub unsafe fn test_180() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_178));
    (unsafe { testExtents_126() });
    (unsafe { testExtents_127() });
    (unsafe { testExtents_128() });
    (unsafe { testExtents_129() });
    (unsafe { testExtents_130() });
    (unsafe { testExtents_131() });
    (unsafe { testExtents_132() });
    (unsafe { testExtents_133() });
    (unsafe { testExtents_134() });
    (unsafe { testExtents_135() });
    (unsafe { testExtents_136() });
    (unsafe { testExtents_137() });
    (unsafe { testExtents_138() });
    (unsafe { testExtents_139() });
    (unsafe { testExtents_140() });
    (unsafe { testExtents_141() });
    (unsafe { testExtents_142() });
}
pub unsafe fn test_181() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_178));
    (unsafe { testExtents_143() });
    (unsafe { testExtents_144() });
    (unsafe { testExtents_145() });
    (unsafe { testExtents_146() });
    (unsafe { testExtents_147() });
    (unsafe { testExtents_148() });
    (unsafe { testExtents_149() });
    (unsafe { testExtents_150() });
    (unsafe { testExtents_151() });
    (unsafe { testExtents_152() });
    (unsafe { testExtents_153() });
    (unsafe { testExtents_154() });
    (unsafe { testExtents_155() });
    (unsafe { testExtents_156() });
    (unsafe { testExtents_157() });
    (unsafe { testExtents_158() });
    (unsafe { testExtents_159() });
}
pub unsafe fn test_182() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_178));
    (unsafe { testExtents_160() });
    (unsafe { testExtents_161() });
    (unsafe { testExtents_162() });
    (unsafe { testExtents_163() });
    (unsafe { testExtents_164() });
    (unsafe { testExtents_165() });
    (unsafe { testExtents_166() });
    (unsafe { testExtents_167() });
    (unsafe { testExtents_168() });
    (unsafe { testExtents_169() });
    (unsafe { testExtents_170() });
    (unsafe { testExtents_171() });
    (unsafe { testExtents_172() });
    (unsafe { testExtents_173() });
    (unsafe { testExtents_174() });
    (unsafe { testExtents_175() });
    (unsafe { testExtents_176() });
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
    (unsafe { test_177() });
    (unsafe { test_179() });
    (unsafe { test_180() });
    (unsafe { test_181() });
    (unsafe { test_182() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
