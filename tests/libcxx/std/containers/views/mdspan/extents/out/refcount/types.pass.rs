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
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_long__long_long_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long_long__unsigned_long_long_ {}
pub fn testExtents_7() {
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
pub fn testExtents_8() {
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
pub fn testExtents_9() {
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
pub fn testExtents_10() {
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
pub fn testExtents_11() {
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
pub fn testExtents_12() {
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
pub fn testExtents_13() {
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
pub fn testExtents_14() {
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
pub fn testExtents_15() {
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
pub fn testExtents_16() {
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
pub fn testExtents_17() {
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
pub fn testExtents_18() {
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
pub fn testExtents_19() {
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
pub fn testExtents_20() {
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
pub fn testExtents_21() {
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
pub fn testExtents_22() {
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
pub fn testExtents_23() {
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
pub fn testExtents_24() {
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
pub fn testExtents_25() {
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
pub fn testExtents_26() {
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
pub fn testExtents_27() {
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
pub fn testExtents_28() {
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
pub fn testExtents_29() {
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
pub fn testExtents_30() {
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
pub fn testExtents_31() {
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
pub fn testExtents_32() {
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
pub fn testExtents_33() {
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
pub fn testExtents_34() {
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
pub fn testExtents_35() {
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
pub fn testExtents_36() {
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
pub fn testExtents_37() {
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
pub fn testExtents_38() {
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
pub fn testExtents_39() {
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
pub fn testExtents_40() {
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
pub fn testExtents_41() {
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
pub fn testExtents_42() {
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
pub fn testExtents_43() {
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
pub fn testExtents_44() {
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
pub fn testExtents_45() {
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
pub fn testExtents_46() {
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
pub fn testExtents_47() {
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
pub fn testExtents_48() {
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
pub fn testExtents_49() {
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
pub fn testExtents_50() {
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
pub fn testExtents_51() {
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
pub fn testExtents_52() {
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
pub fn testExtents_53() {
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
pub fn testExtents_54() {
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
pub fn testExtents_55() {
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
pub fn testExtents_56() {
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
pub fn testExtents_57() {
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
pub fn testExtents_58() {
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
pub fn testExtents_59() {
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
pub fn testExtents_60() {
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
pub fn testExtents_61() {
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
pub fn testExtents_62() {
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
pub fn testExtents_63() {
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
pub fn testExtents_64() {
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
pub fn testExtents_65() {
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
pub fn testExtents_66() {
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
pub fn testExtents_67() {
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
pub fn testExtents_68() {
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
pub fn testExtents_69() {
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
pub fn testExtents_70() {
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
pub fn testExtents_71() {
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
pub fn testExtents_72() {
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
pub fn testExtents_73() {
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
pub fn testExtents_74() {
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
pub fn testExtents_75() {
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
pub fn testExtents_76() {
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
pub fn testExtents_77() {
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
pub fn testExtents_78() {
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
pub fn testExtents_79() {
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
pub fn testExtents_80() {
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
pub fn testExtents_81() {
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
pub fn testExtents_82() {
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
pub fn testExtents_83() {
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
pub fn testExtents_84() {
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
pub fn testExtents_85() {
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
pub fn testExtents_86() {
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
pub fn testExtents_87() {
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
pub fn testExtents_88() {
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
pub fn testExtents_89() {
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
pub fn testExtents_90() {
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
pub fn testExtents_91() {
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
pub fn testExtents_92() {
    ({ testExtents_7() });
}
pub fn testExtents_93() {
    ({ testExtents_8() });
}
pub fn testExtents_94() {
    ({ testExtents_9() });
}
pub fn testExtents_95() {
    ({ testExtents_10() });
}
pub fn testExtents_96() {
    ({ testExtents_11() });
}
pub fn testExtents_97() {
    ({ testExtents_12() });
}
pub fn testExtents_98() {
    ({ testExtents_13() });
}
pub fn testExtents_99() {
    ({ testExtents_14() });
}
pub fn testExtents_100() {
    ({ testExtents_15() });
}
pub fn testExtents_101() {
    ({ testExtents_16() });
}
pub fn testExtents_102() {
    ({ testExtents_17() });
}
pub fn testExtents_103() {
    ({ testExtents_18() });
}
pub fn testExtents_104() {
    ({ testExtents_19() });
}
pub fn testExtents_105() {
    ({ testExtents_20() });
}
pub fn testExtents_106() {
    ({ testExtents_21() });
}
pub fn testExtents_107() {
    ({ testExtents_22() });
}
pub fn testExtents_108() {
    ({ testExtents_23() });
}
pub fn testExtents_109() {
    ({ testExtents_24() });
}
pub fn testExtents_110() {
    ({ testExtents_25() });
}
pub fn testExtents_111() {
    ({ testExtents_26() });
}
pub fn testExtents_112() {
    ({ testExtents_27() });
}
pub fn testExtents_113() {
    ({ testExtents_28() });
}
pub fn testExtents_114() {
    ({ testExtents_29() });
}
pub fn testExtents_115() {
    ({ testExtents_30() });
}
pub fn testExtents_116() {
    ({ testExtents_31() });
}
pub fn testExtents_117() {
    ({ testExtents_32() });
}
pub fn testExtents_118() {
    ({ testExtents_33() });
}
pub fn testExtents_119() {
    ({ testExtents_34() });
}
pub fn testExtents_120() {
    ({ testExtents_35() });
}
pub fn testExtents_121() {
    ({ testExtents_36() });
}
pub fn testExtents_122() {
    ({ testExtents_37() });
}
pub fn testExtents_123() {
    ({ testExtents_38() });
}
pub fn testExtents_124() {
    ({ testExtents_39() });
}
pub fn testExtents_125() {
    ({ testExtents_40() });
}
pub fn testExtents_126() {
    ({ testExtents_41() });
}
pub fn testExtents_127() {
    ({ testExtents_42() });
}
pub fn testExtents_128() {
    ({ testExtents_43() });
}
pub fn testExtents_129() {
    ({ testExtents_44() });
}
pub fn testExtents_130() {
    ({ testExtents_45() });
}
pub fn testExtents_131() {
    ({ testExtents_46() });
}
pub fn testExtents_132() {
    ({ testExtents_47() });
}
pub fn testExtents_133() {
    ({ testExtents_48() });
}
pub fn testExtents_134() {
    ({ testExtents_49() });
}
pub fn testExtents_135() {
    ({ testExtents_50() });
}
pub fn testExtents_136() {
    ({ testExtents_51() });
}
pub fn testExtents_137() {
    ({ testExtents_52() });
}
pub fn testExtents_138() {
    ({ testExtents_53() });
}
pub fn testExtents_139() {
    ({ testExtents_54() });
}
pub fn testExtents_140() {
    ({ testExtents_55() });
}
pub fn testExtents_141() {
    ({ testExtents_56() });
}
pub fn testExtents_142() {
    ({ testExtents_57() });
}
pub fn testExtents_143() {
    ({ testExtents_58() });
}
pub fn testExtents_144() {
    ({ testExtents_59() });
}
pub fn testExtents_145() {
    ({ testExtents_60() });
}
pub fn testExtents_146() {
    ({ testExtents_61() });
}
pub fn testExtents_147() {
    ({ testExtents_62() });
}
pub fn testExtents_148() {
    ({ testExtents_63() });
}
pub fn testExtents_149() {
    ({ testExtents_64() });
}
pub fn testExtents_150() {
    ({ testExtents_65() });
}
pub fn testExtents_151() {
    ({ testExtents_66() });
}
pub fn testExtents_152() {
    ({ testExtents_67() });
}
pub fn testExtents_153() {
    ({ testExtents_68() });
}
pub fn testExtents_154() {
    ({ testExtents_69() });
}
pub fn testExtents_155() {
    ({ testExtents_70() });
}
pub fn testExtents_156() {
    ({ testExtents_71() });
}
pub fn testExtents_157() {
    ({ testExtents_72() });
}
pub fn testExtents_158() {
    ({ testExtents_73() });
}
pub fn testExtents_159() {
    ({ testExtents_74() });
}
pub fn testExtents_160() {
    ({ testExtents_75() });
}
pub fn testExtents_161() {
    ({ testExtents_76() });
}
pub fn testExtents_162() {
    ({ testExtents_77() });
}
pub fn testExtents_163() {
    ({ testExtents_78() });
}
pub fn testExtents_164() {
    ({ testExtents_79() });
}
pub fn testExtents_165() {
    ({ testExtents_80() });
}
pub fn testExtents_166() {
    ({ testExtents_81() });
}
pub fn testExtents_167() {
    ({ testExtents_82() });
}
pub fn testExtents_168() {
    ({ testExtents_83() });
}
pub fn testExtents_169() {
    ({ testExtents_84() });
}
pub fn testExtents_170() {
    ({ testExtents_85() });
}
pub fn testExtents_171() {
    ({ testExtents_86() });
}
pub fn testExtents_172() {
    ({ testExtents_87() });
}
pub fn testExtents_173() {
    ({ testExtents_88() });
}
pub fn testExtents_174() {
    ({ testExtents_89() });
}
pub fn testExtents_175() {
    ({ testExtents_90() });
}
pub fn testExtents_176() {
    ({ testExtents_91() });
}
pub fn test_177() {
    let D: Value<()> = Rc::new(RefCell::new(dynamic_extent_178.with(|rc| *rc.borrow())));
    ({ testExtents_92() });
    ({ testExtents_93() });
    ({ testExtents_94() });
    ({ testExtents_95() });
    ({ testExtents_96() });
    ({ testExtents_97() });
    ({ testExtents_98() });
    ({ testExtents_99() });
    ({ testExtents_100() });
    ({ testExtents_101() });
    ({ testExtents_102() });
    ({ testExtents_103() });
    ({ testExtents_104() });
    ({ testExtents_105() });
    ({ testExtents_106() });
    ({ testExtents_107() });
    ({ testExtents_108() });
}
pub fn test_179() {
    let D: Value<()> = Rc::new(RefCell::new(dynamic_extent_178.with(|rc| *rc.borrow())));
    ({ testExtents_109() });
    ({ testExtents_110() });
    ({ testExtents_111() });
    ({ testExtents_112() });
    ({ testExtents_113() });
    ({ testExtents_114() });
    ({ testExtents_115() });
    ({ testExtents_116() });
    ({ testExtents_117() });
    ({ testExtents_118() });
    ({ testExtents_119() });
    ({ testExtents_120() });
    ({ testExtents_121() });
    ({ testExtents_122() });
    ({ testExtents_123() });
    ({ testExtents_124() });
    ({ testExtents_125() });
}
pub fn test_180() {
    let D: Value<()> = Rc::new(RefCell::new(dynamic_extent_178.with(|rc| *rc.borrow())));
    ({ testExtents_126() });
    ({ testExtents_127() });
    ({ testExtents_128() });
    ({ testExtents_129() });
    ({ testExtents_130() });
    ({ testExtents_131() });
    ({ testExtents_132() });
    ({ testExtents_133() });
    ({ testExtents_134() });
    ({ testExtents_135() });
    ({ testExtents_136() });
    ({ testExtents_137() });
    ({ testExtents_138() });
    ({ testExtents_139() });
    ({ testExtents_140() });
    ({ testExtents_141() });
    ({ testExtents_142() });
}
pub fn test_181() {
    let D: Value<()> = Rc::new(RefCell::new(dynamic_extent_178.with(|rc| *rc.borrow())));
    ({ testExtents_143() });
    ({ testExtents_144() });
    ({ testExtents_145() });
    ({ testExtents_146() });
    ({ testExtents_147() });
    ({ testExtents_148() });
    ({ testExtents_149() });
    ({ testExtents_150() });
    ({ testExtents_151() });
    ({ testExtents_152() });
    ({ testExtents_153() });
    ({ testExtents_154() });
    ({ testExtents_155() });
    ({ testExtents_156() });
    ({ testExtents_157() });
    ({ testExtents_158() });
    ({ testExtents_159() });
}
pub fn test_182() {
    let D: Value<()> = Rc::new(RefCell::new(dynamic_extent_178.with(|rc| *rc.borrow())));
    ({ testExtents_160() });
    ({ testExtents_161() });
    ({ testExtents_162() });
    ({ testExtents_163() });
    ({ testExtents_164() });
    ({ testExtents_165() });
    ({ testExtents_166() });
    ({ testExtents_167() });
    ({ testExtents_168() });
    ({ testExtents_169() });
    ({ testExtents_170() });
    ({ testExtents_171() });
    ({ testExtents_172() });
    ({ testExtents_173() });
    ({ testExtents_174() });
    ({ testExtents_175() });
    ({ testExtents_176() });
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
    ({ test_177() });
    ({ test_179() });
    ({ test_180() });
    ({ test_181() });
    ({ test_182() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
