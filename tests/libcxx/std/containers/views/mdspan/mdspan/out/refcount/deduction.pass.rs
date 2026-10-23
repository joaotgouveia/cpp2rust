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
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_int___std_layout_left__std_default_accessor_int____std_mdspan_int__std_extents_int___std_layout_left__std_default_accessor_int___
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_int___
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_int___
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_int___
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_int___
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_int____std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_int___
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_int___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_int___std_layout_right__std_default_accessor_int___
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_int___
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_int___layout_wrapping_integral_4___std_default_accessor_int____std_mdspan_int__std_extents_int___layout_wrapping_integral_4___std_default_accessor_int___
{}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int___
{}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_int____std_mdspan_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_int___
{}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int___
{}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int___
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int____std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_int___
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long___std_layout_right__std_default_accessor_int___
{}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_20 = ();
pub const anon_20_value: anon_20 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_int___
{}
pub type anon_21 = ();
pub const anon_21_value: anon_21 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_22 = ();
pub const anon_22_value: anon_22 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_int___
{}
pub type anon_23 = ();
pub const anon_23_value: anon_23 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_int___
{}
pub type anon_24 = ();
pub const anon_24_value: anon_24 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_int___std_layout_left__checked_accessor_int____std_mdspan_int__std_extents_int___std_layout_left__checked_accessor_int___
{}
pub type anon_25 = ();
pub const anon_25_value: anon_25 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_int___
{}
pub type anon_26 = ();
pub const anon_26_value: anon_26 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_int__7___std_layout_left__checked_accessor_int____std_mdspan_int__std_extents_unsigned_int__7___std_layout_left__checked_accessor_int___
{}
pub type anon_27 = ();
pub const anon_27_value: anon_27 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_int___
{}
pub type anon_28 = ();
pub const anon_28_value: anon_28 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_int___
{}
pub type anon_29 = ();
pub const anon_29_value: anon_29 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_int____std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_int___
{}
pub type anon_30 = ();
pub const anon_30_value: anon_30 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_int___std_layout_right__checked_accessor_int____std_mdspan_int__std_extents_int___std_layout_right__checked_accessor_int___
{}
pub type anon_31 = ();
pub const anon_31_value: anon_31 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_int___
{}
pub type anon_32 = ();
pub const anon_32_value: anon_32 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_int__7___std_layout_right__checked_accessor_int____std_mdspan_int__std_extents_unsigned_int__7___std_layout_right__checked_accessor_int___
{}
pub type anon_33 = ();
pub const anon_33_value: anon_33 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_int___
{}
pub type anon_34 = ();
pub const anon_34_value: anon_34 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_int___
{}
pub type anon_35 = ();
pub const anon_35_value: anon_35 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_int____std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_int___
{}
pub type anon_36 = ();
pub const anon_36_value: anon_36 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_int___layout_wrapping_integral_4___checked_accessor_int____std_mdspan_int__std_extents_int___layout_wrapping_integral_4___checked_accessor_int___
{}
pub type anon_37 = ();
pub const anon_37_value: anon_37 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int___
{}
pub type anon_38 = ();
pub const anon_38_value: anon_38 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_int____std_mdspan_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_int___
{}
pub type anon_39 = ();
pub const anon_39_value: anon_39 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int____std_mdspan_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int___
{}
pub type anon_40 = ();
pub const anon_40_value: anon_40 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int____std_mdspan_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int___
{}
pub type anon_41 = ();
pub const anon_41_value: anon_41 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int____std_mdspan_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_int___
{}
pub type anon_42 = ();
pub const anon_42_value: anon_42 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_int___std_layout_left__std_default_accessor_const_int____std_mdspan_const_int__std_extents_int___std_layout_left__std_default_accessor_const_int___
{}
pub type anon_43 = ();
pub const anon_43_value: anon_43 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_const_int___
{}
pub type anon_44 = ();
pub const anon_44_value: anon_44 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_const_int___
{}
pub type anon_45 = ();
pub const anon_45_value: anon_45 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_const_int___
{}
pub type anon_46 = ();
pub const anon_46_value: anon_46 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_const_int___
{}
pub type anon_47 = ();
pub const anon_47_value: anon_47 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_const_int____std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_const_int___
{}
pub type anon_48 = ();
pub const anon_48_value: anon_48 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_int___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_int___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_49 = ();
pub const anon_49_value: anon_49 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_50 = ();
pub const anon_50_value: anon_50 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_51 = ();
pub const anon_51_value: anon_51 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_52 = ();
pub const anon_52_value: anon_52 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_53 = ();
pub const anon_53_value: anon_53 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_54 = ();
pub const anon_54_value: anon_54 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_int___layout_wrapping_integral_4___std_default_accessor_const_int____std_mdspan_const_int__std_extents_int___layout_wrapping_integral_4___std_default_accessor_const_int___
{}
pub type anon_55 = ();
pub const anon_55_value: anon_55 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int___
{}
pub type anon_56 = ();
pub const anon_56_value: anon_56 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_const_int___
{}
pub type anon_57 = ();
pub const anon_57_value: anon_57 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int___
{}
pub type anon_58 = ();
pub const anon_58_value: anon_58 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int___
{}
pub type anon_59 = ();
pub const anon_59_value: anon_59 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int____std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_int___
{}
pub type anon_60 = ();
pub const anon_60_value: anon_60 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_61 = ();
pub const anon_61_value: anon_61 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_62 = ();
pub const anon_62_value: anon_62 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_63 = ();
pub const anon_63_value: anon_63 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_64 = ();
pub const anon_64_value: anon_64 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_65 = ();
pub const anon_65_value: anon_65 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_int___
{}
pub type anon_66 = ();
pub const anon_66_value: anon_66 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_int___std_layout_left__checked_accessor_const_int____std_mdspan_const_int__std_extents_int___std_layout_left__checked_accessor_const_int___
{}
pub type anon_67 = ();
pub const anon_67_value: anon_67 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_const_int___
{}
pub type anon_68 = ();
pub const anon_68_value: anon_68 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_left__checked_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_left__checked_accessor_const_int___
{}
pub type anon_69 = ();
pub const anon_69_value: anon_69 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_const_int___
{}
pub type anon_70 = ();
pub const anon_70_value: anon_70 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_const_int___
{}
pub type anon_71 = ();
pub const anon_71_value: anon_71 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_const_int____std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_const_int___
{}
pub type anon_72 = ();
pub const anon_72_value: anon_72 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_int___std_layout_right__checked_accessor_const_int____std_mdspan_const_int__std_extents_int___std_layout_right__checked_accessor_const_int___
{}
pub type anon_73 = ();
pub const anon_73_value: anon_73 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_const_int___
{}
pub type anon_74 = ();
pub const anon_74_value: anon_74 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_right__checked_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_int__7___std_layout_right__checked_accessor_const_int___
{}
pub type anon_75 = ();
pub const anon_75_value: anon_75 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_const_int___
{}
pub type anon_76 = ();
pub const anon_76_value: anon_76 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_const_int___
{}
pub type anon_77 = ();
pub const anon_77_value: anon_77 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_const_int____std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_const_int___
{}
pub type anon_78 = ();
pub const anon_78_value: anon_78 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_int___layout_wrapping_integral_4___checked_accessor_const_int____std_mdspan_const_int__std_extents_int___layout_wrapping_integral_4___checked_accessor_const_int___
{}
pub type anon_79 = ();
pub const anon_79_value: anon_79 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int___
{}
pub type anon_80 = ();
pub const anon_80_value: anon_80 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_const_int___
{}
pub type anon_81 = ();
pub const anon_81_value: anon_81 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int____std_mdspan_const_int__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int___
{}
pub type anon_82 = ();
pub const anon_82_value: anon_82 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int____std_mdspan_const_int__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int___
{}
pub type anon_83 = ();
pub const anon_83_value: anon_83 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int____std_mdspan_const_int__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_int___
{}
pub type anon_84 = ();
pub const anon_84_value: anon_84 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_int___std_layout_left__std_default_accessor_double____std_mdspan_double__std_extents_int___std_layout_left__std_default_accessor_double___
{}
pub type anon_85 = ();
pub const anon_85_value: anon_85 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_double___
{}
pub type anon_86 = ();
pub const anon_86_value: anon_86 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_double___
{}
pub type anon_87 = ();
pub const anon_87_value: anon_87 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_double___
{}
pub type anon_88 = ();
pub const anon_88_value: anon_88 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_double___
{}
pub type anon_89 = ();
pub const anon_89_value: anon_89 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_double____std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_double___
{}
pub type anon_90 = ();
pub const anon_90_value: anon_90 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_int___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_int___std_layout_right__std_default_accessor_double___
{}
pub type anon_91 = ();
pub const anon_91_value: anon_91 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_92 = ();
pub const anon_92_value: anon_92 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_double___
{}
pub type anon_93 = ();
pub const anon_93_value: anon_93 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_94 = ();
pub const anon_94_value: anon_94 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_95 = ();
pub const anon_95_value: anon_95 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_96 = ();
pub const anon_96_value: anon_96 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_int___layout_wrapping_integral_4___std_default_accessor_double____std_mdspan_double__std_extents_int___layout_wrapping_integral_4___std_default_accessor_double___
{}
pub type anon_97 = ();
pub const anon_97_value: anon_97 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double___
{}
pub type anon_98 = ();
pub const anon_98_value: anon_98 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_double____std_mdspan_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_double___
{}
pub type anon_99 = ();
pub const anon_99_value: anon_99 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double___
{}
pub type anon_100 = ();
pub const anon_100_value: anon_100 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double___
{}
pub type anon_101 = ();
pub const anon_101_value: anon_101 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double____std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_double___
{}
pub type anon_102 = ();
pub const anon_102_value: anon_102 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long___std_layout_right__std_default_accessor_double___
{}
pub type anon_103 = ();
pub const anon_103_value: anon_103 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_104 = ();
pub const anon_104_value: anon_104 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_double___
{}
pub type anon_105 = ();
pub const anon_105_value: anon_105 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_106 = ();
pub const anon_106_value: anon_106 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_double___
{}
pub type anon_107 = ();
pub const anon_107_value: anon_107 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_double___
{}
pub type anon_108 = ();
pub const anon_108_value: anon_108 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_int___std_layout_left__checked_accessor_double____std_mdspan_double__std_extents_int___std_layout_left__checked_accessor_double___
{}
pub type anon_109 = ();
pub const anon_109_value: anon_109 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_double___
{}
pub type anon_110 = ();
pub const anon_110_value: anon_110 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_int__7___std_layout_left__checked_accessor_double____std_mdspan_double__std_extents_unsigned_int__7___std_layout_left__checked_accessor_double___
{}
pub type anon_111 = ();
pub const anon_111_value: anon_111 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_double___
{}
pub type anon_112 = ();
pub const anon_112_value: anon_112 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_double___
{}
pub type anon_113 = ();
pub const anon_113_value: anon_113 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_double____std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_double___
{}
pub type anon_114 = ();
pub const anon_114_value: anon_114 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_int___std_layout_right__checked_accessor_double____std_mdspan_double__std_extents_int___std_layout_right__checked_accessor_double___
{}
pub type anon_115 = ();
pub const anon_115_value: anon_115 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_double___
{}
pub type anon_116 = ();
pub const anon_116_value: anon_116 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_int__7___std_layout_right__checked_accessor_double____std_mdspan_double__std_extents_unsigned_int__7___std_layout_right__checked_accessor_double___
{}
pub type anon_117 = ();
pub const anon_117_value: anon_117 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_double___
{}
pub type anon_118 = ();
pub const anon_118_value: anon_118 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_double___
{}
pub type anon_119 = ();
pub const anon_119_value: anon_119 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_double____std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_double___
{}
pub type anon_120 = ();
pub const anon_120_value: anon_120 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_int___layout_wrapping_integral_4___checked_accessor_double____std_mdspan_double__std_extents_int___layout_wrapping_integral_4___checked_accessor_double___
{}
pub type anon_121 = ();
pub const anon_121_value: anon_121 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double___
{}
pub type anon_122 = ();
pub const anon_122_value: anon_122 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_double____std_mdspan_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_double___
{}
pub type anon_123 = ();
pub const anon_123_value: anon_123 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double____std_mdspan_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double___
{}
pub type anon_124 = ();
pub const anon_124_value: anon_124 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double____std_mdspan_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double___
{}
pub type anon_125 = ();
pub const anon_125_value: anon_125 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double____std_mdspan_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_double___
{}
pub type anon_126 = ();
pub const anon_126_value: anon_126 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_int___std_layout_left__std_default_accessor_const_double____std_mdspan_const_double__std_extents_int___std_layout_left__std_default_accessor_const_double___
{}
pub type anon_127 = ();
pub const anon_127_value: anon_127 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_const_double___
{}
pub type anon_128 = ();
pub const anon_128_value: anon_128 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_const_double___
{}
pub type anon_129 = ();
pub const anon_129_value: anon_129 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_const_double___
{}
pub type anon_130 = ();
pub const anon_130_value: anon_130 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_const_double___
{}
pub type anon_131 = ();
pub const anon_131_value: anon_131 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_const_double____std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_const_double___
{}
pub type anon_132 = ();
pub const anon_132_value: anon_132 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_int___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_int___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_133 = ();
pub const anon_133_value: anon_133 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_134 = ();
pub const anon_134_value: anon_134 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_135 = ();
pub const anon_135_value: anon_135 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_136 = ();
pub const anon_136_value: anon_136 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_137 = ();
pub const anon_137_value: anon_137 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_138 = ();
pub const anon_138_value: anon_138 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_int___layout_wrapping_integral_4___std_default_accessor_const_double____std_mdspan_const_double__std_extents_int___layout_wrapping_integral_4___std_default_accessor_const_double___
{}
pub type anon_139 = ();
pub const anon_139_value: anon_139 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double___
{}
pub type anon_140 = ();
pub const anon_140_value: anon_140 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_const_double___
{}
pub type anon_141 = ();
pub const anon_141_value: anon_141 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double___
{}
pub type anon_142 = ();
pub const anon_142_value: anon_142 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double___
{}
pub type anon_143 = ();
pub const anon_143_value: anon_143 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double____std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_double___
{}
pub type anon_144 = ();
pub const anon_144_value: anon_144 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_145 = ();
pub const anon_145_value: anon_145 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_146 = ();
pub const anon_146_value: anon_146 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_147 = ();
pub const anon_147_value: anon_147 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_148 = ();
pub const anon_148_value: anon_148 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_149 = ();
pub const anon_149_value: anon_149 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_double___
{}
pub type anon_150 = ();
pub const anon_150_value: anon_150 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_int___std_layout_left__checked_accessor_const_double____std_mdspan_const_double__std_extents_int___std_layout_left__checked_accessor_const_double___
{}
pub type anon_151 = ();
pub const anon_151_value: anon_151 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_const_double___
{}
pub type anon_152 = ();
pub const anon_152_value: anon_152 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_left__checked_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_left__checked_accessor_const_double___
{}
pub type anon_153 = ();
pub const anon_153_value: anon_153 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_const_double___
{}
pub type anon_154 = ();
pub const anon_154_value: anon_154 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_const_double___
{}
pub type anon_155 = ();
pub const anon_155_value: anon_155 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_const_double____std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_const_double___
{}
pub type anon_156 = ();
pub const anon_156_value: anon_156 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_int___std_layout_right__checked_accessor_const_double____std_mdspan_const_double__std_extents_int___std_layout_right__checked_accessor_const_double___
{}
pub type anon_157 = ();
pub const anon_157_value: anon_157 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_const_double___
{}
pub type anon_158 = ();
pub const anon_158_value: anon_158 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_right__checked_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_int__7___std_layout_right__checked_accessor_const_double___
{}
pub type anon_159 = ();
pub const anon_159_value: anon_159 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_const_double___
{}
pub type anon_160 = ();
pub const anon_160_value: anon_160 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_const_double___
{}
pub type anon_161 = ();
pub const anon_161_value: anon_161 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_const_double____std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_const_double___
{}
pub type anon_162 = ();
pub const anon_162_value: anon_162 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_int___layout_wrapping_integral_4___checked_accessor_const_double____std_mdspan_const_double__std_extents_int___layout_wrapping_integral_4___checked_accessor_const_double___
{}
pub type anon_163 = ();
pub const anon_163_value: anon_163 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double___
{}
pub type anon_164 = ();
pub const anon_164_value: anon_164 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_const_double___
{}
pub type anon_165 = ();
pub const anon_165_value: anon_165 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double____std_mdspan_const_double__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double___
{}
pub type anon_166 = ();
pub const anon_166_value: anon_166 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double____std_mdspan_const_double__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double___
{}
pub type anon_167 = ();
pub const anon_167_value: anon_167 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double____std_mdspan_const_double__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_double___
{}
pub type anon_168 = ();
pub const anon_168_value: anon_168 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_int___std_layout_left__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_int___std_layout_left__std_default_accessor_MinimalElementType___
{}
pub type anon_169 = ();
pub const anon_169_value: anon_169 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType___
{}
pub type anon_170 = ();
pub const anon_170_value: anon_170 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_MinimalElementType___
{}
pub type anon_171 = ();
pub const anon_171_value: anon_171 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType___
{}
pub type anon_172 = ();
pub const anon_172_value: anon_172 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType___
{}
pub type anon_173 = ();
pub const anon_173_value: anon_173 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_MinimalElementType___
{}
pub type anon_174 = ();
pub const anon_174_value: anon_174 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_int___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_int___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_175 = ();
pub const anon_175_value: anon_175 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_176 = ();
pub const anon_176_value: anon_176 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_177 = ();
pub const anon_177_value: anon_177 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_178 = ();
pub const anon_178_value: anon_178 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_179 = ();
pub const anon_179_value: anon_179 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_180 = ();
pub const anon_180_value: anon_180 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_int___layout_wrapping_integral_4___std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_int___layout_wrapping_integral_4___std_default_accessor_MinimalElementType___
{}
pub type anon_181 = ();
pub const anon_181_value: anon_181 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType___
{}
pub type anon_182 = ();
pub const anon_182_value: anon_182 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_MinimalElementType___
{}
pub type anon_183 = ();
pub const anon_183_value: anon_183 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType___
{}
pub type anon_184 = ();
pub const anon_184_value: anon_184 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType___
{}
pub type anon_185 = ();
pub const anon_185_value: anon_185 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_MinimalElementType___
{}
pub type anon_186 = ();
pub const anon_186_value: anon_186 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_187 = ();
pub const anon_187_value: anon_187 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_188 = ();
pub const anon_188_value: anon_188 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_189 = ();
pub const anon_189_value: anon_189 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_190 = ();
pub const anon_190_value: anon_190 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_191 = ();
pub const anon_191_value: anon_191 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_MinimalElementType___
{}
pub type anon_192 = ();
pub const anon_192_value: anon_192 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_int___std_layout_left__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_int___std_layout_left__checked_accessor_MinimalElementType___
{}
pub type anon_193 = ();
pub const anon_193_value: anon_193 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType___
{}
pub type anon_194 = ();
pub const anon_194_value: anon_194 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__checked_accessor_MinimalElementType___
{}
pub type anon_195 = ();
pub const anon_195_value: anon_195 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType___
{}
pub type anon_196 = ();
pub const anon_196_value: anon_196 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType___
{}
pub type anon_197 = ();
pub const anon_197_value: anon_197 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_MinimalElementType___
{}
pub type anon_198 = ();
pub const anon_198_value: anon_198 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_int___std_layout_right__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_int___std_layout_right__checked_accessor_MinimalElementType___
{}
pub type anon_199 = ();
pub const anon_199_value: anon_199 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType___
{}
pub type anon_200 = ();
pub const anon_200_value: anon_200 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__checked_accessor_MinimalElementType___
{}
pub type anon_201 = ();
pub const anon_201_value: anon_201 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType___
{}
pub type anon_202 = ();
pub const anon_202_value: anon_202 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType___
{}
pub type anon_203 = ();
pub const anon_203_value: anon_203 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_MinimalElementType___
{}
pub type anon_204 = ();
pub const anon_204_value: anon_204 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_int___layout_wrapping_integral_4___checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_int___layout_wrapping_integral_4___checked_accessor_MinimalElementType___
{}
pub type anon_205 = ();
pub const anon_205_value: anon_205 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType___
{}
pub type anon_206 = ();
pub const anon_206_value: anon_206 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_MinimalElementType___
{}
pub type anon_207 = ();
pub const anon_207_value: anon_207 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType___
{}
pub type anon_208 = ();
pub const anon_208_value: anon_208 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType___
{}
pub type anon_209 = ();
pub const anon_209_value: anon_209 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType____std_mdspan_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_MinimalElementType___
{}
pub type anon_210 = ();
pub const anon_210_value: anon_210 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_int___std_layout_left__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_int___std_layout_left__std_default_accessor_const_MinimalElementType___
{}
pub type anon_211 = ();
pub const anon_211_value: anon_211 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType___
{}
pub type anon_212 = ();
pub const anon_212_value: anon_212 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__std_default_accessor_const_MinimalElementType___
{}
pub type anon_213 = ();
pub const anon_213_value: anon_213 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType___
{}
pub type anon_214 = ();
pub const anon_214_value: anon_214 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType___
{}
pub type anon_215 = ();
pub const anon_215_value: anon_215 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__std_default_accessor_const_MinimalElementType___
{}
pub type anon_216 = ();
pub const anon_216_value: anon_216 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_int___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_int___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_217 = ();
pub const anon_217_value: anon_217 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_218 = ();
pub const anon_218_value: anon_218 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_219 = ();
pub const anon_219_value: anon_219 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_220 = ();
pub const anon_220_value: anon_220 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_221 = ();
pub const anon_221_value: anon_221 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_222 = ();
pub const anon_222_value: anon_222 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_int___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_int___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType___
{}
pub type anon_223 = ();
pub const anon_223_value: anon_223 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType___
{}
pub type anon_224 = ();
pub const anon_224_value: anon_224 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType___
{}
pub type anon_225 = ();
pub const anon_225_value: anon_225 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType___
{}
pub type anon_226 = ();
pub const anon_226_value: anon_226 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType___
{}
pub type anon_227 = ();
pub const anon_227_value: anon_227 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___std_default_accessor_const_MinimalElementType___
{}
pub type anon_228 = ();
pub const anon_228_value: anon_228 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_229 = ();
pub const anon_229_value: anon_229 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_230 = ();
pub const anon_230_value: anon_230 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__5___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_231 = ();
pub const anon_231_value: anon_231 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_232 = ();
pub const anon_232_value: anon_232 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__5__18446744073709551615__7___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_233 = ();
pub const anon_233_value: anon_233 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615___std_layout_right__std_default_accessor_const_MinimalElementType___
{}
pub type anon_234 = ();
pub const anon_234_value: anon_234 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_int___std_layout_left__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_int___std_layout_left__checked_accessor_const_MinimalElementType___
{}
pub type anon_235 = ();
pub const anon_235_value: anon_235 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType___
{}
pub type anon_236 = ();
pub const anon_236_value: anon_236 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_left__checked_accessor_const_MinimalElementType___
{}
pub type anon_237 = ();
pub const anon_237_value: anon_237 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType___
{}
pub type anon_238 = ();
pub const anon_238_value: anon_238 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType___
{}
pub type anon_239 = ();
pub const anon_239_value: anon_239 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_left__checked_accessor_const_MinimalElementType___
{}
pub type anon_240 = ();
pub const anon_240_value: anon_240 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_int___std_layout_right__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_int___std_layout_right__checked_accessor_const_MinimalElementType___
{}
pub type anon_241 = ();
pub const anon_241_value: anon_241 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType___
{}
pub type anon_242 = ();
pub const anon_242_value: anon_242 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___std_layout_right__checked_accessor_const_MinimalElementType___
{}
pub type anon_243 = ();
pub const anon_243_value: anon_243 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType___
{}
pub type anon_244 = ();
pub const anon_244_value: anon_244 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType___
{}
pub type anon_245 = ();
pub const anon_245_value: anon_245 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___std_layout_right__checked_accessor_const_MinimalElementType___
{}
pub type anon_246 = ();
pub const anon_246_value: anon_246 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_int___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_int___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType___
{}
pub type anon_247 = ();
pub const anon_247_value: anon_247 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType___
{}
pub type anon_248 = ();
pub const anon_248_value: anon_248 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_int__7___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType___
{}
pub type anon_249 = ();
pub const anon_249_value: anon_249 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType___
{}
pub type anon_250 = ();
pub const anon_250_value: anon_250 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_signed_char__18446744073709551615__7__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType___
{}
pub type anon_251 = ();
pub const anon_251_value: anon_251 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType____std_mdspan_const_MinimalElementType__std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___layout_wrapping_integral_4___checked_accessor_const_MinimalElementType___
{}
pub type anon_252 = ();
pub const anon_252_value: anon_252 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_float__std_extents_unsigned_long__12___std_layout_right__std_default_accessor_float____std_mdspan_float__std_extents_unsigned_long__12___std_layout_right__std_default_accessor_float___
{}
pub type anon_253 = ();
pub const anon_253_value: anon_253 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_mdspan_float__std_extents_unsigned_long__1___std_layout_right__std_default_accessor_float____std_mdspan_float__std_extents_unsigned_long__1___std_layout_right__std_default_accessor_float___
{}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MinimalElementType {
    #[offset(0)]
    pub val: (),
}
impl MinimalElementType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_int__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_int__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_int__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_int__1024_> = __this.as_pointer();
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_int__1024_ {
    fn default() -> Self {
        { ElementPool_int__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_const_int__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_const_int__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_const_int__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_const_int__1024_> = __this.as_pointer();
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_const_int__1024_ {
    fn default() -> Self {
        { ElementPool_const_int__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_double__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_double__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_double__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_double__1024_> = __this.as_pointer();
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_double__1024_ {
    fn default() -> Self {
        { ElementPool_double__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_const_double__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_const_double__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_const_double__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_const_double__1024_> = __this.as_pointer();
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_const_double__1024_ {
    fn default() -> Self {
        { ElementPool_const_double__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_MinimalElementType__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MinimalElementType>,
}
impl ElementPool_MinimalElementType__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_MinimalElementType__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<MinimalElementType>::null(),
        }));
        let this: Ptr<ElementPool_MinimalElementType__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_MinimalElementType__1024_ {
    fn default() -> Self {
        { ElementPool_MinimalElementType__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_const_MinimalElementType__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MinimalElementType>,
}
impl ElementPool_const_MinimalElementType__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_const_MinimalElementType__1024_> =
            Rc::new(RefCell::new(Self {
                ptr_: Ptr::<MinimalElementType>::null(),
            }));
        let this: Ptr<ElementPool_const_MinimalElementType__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_const_MinimalElementType__1024_ {
    fn default() -> Self {
        { ElementPool_const_MinimalElementType__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct not_extents_constructible_tag {}
thread_local!(
    static Wrap_254: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_int__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_255: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_256: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_257: Value<()> = Rc::new(RefCell::new((4_() as ())));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_258: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_259: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(32)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4_ {}
pub fn construct_mapping_260(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_261(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_262(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_263(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_264(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_265(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_266(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_267(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_268(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_269(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_270(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_271(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_272(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_273(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_274(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_275 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_276 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_277 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct no_default_ctor_handle_double_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl no_default_ctor_handle_double_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct not_const_convertible_handle_unsigned_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl not_const_convertible_handle_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct not_const_convertible_handle_const_unsigned_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl not_const_convertible_handle_const_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_int_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_int_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_float_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_float_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_double_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_double_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_MinimalElementType_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<MinimalElementType>,
}
impl move_counted_handle_MinimalElementType_ {
    pub fn new_1(mut ptr_: Ptr<MinimalElementType>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_MinimalElementType_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<MinimalElementType>,
}
impl move_counted_handle_const_MinimalElementType_ {
    pub fn new_1(mut ptr_: Ptr<MinimalElementType>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_const_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_int_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_int_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_double_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_double_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_unsigned_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_unsigned_int_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_unsigned_int_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_unsigned_int_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_unsigned_int_> =
            Rc::new(RefCell::new(checked_accessor_unsigned_int_ {
                N: self.N.clone(),
            }));
        checked_accessor_unsigned_int_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_unsigned_int_ {
    fn default() -> Self {
        { checked_accessor_unsigned_int_::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_const_unsigned_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_unsigned_int_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_const_unsigned_int_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_const_unsigned_int_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_const_unsigned_int_> =
            Rc::new(RefCell::new(checked_accessor_const_unsigned_int_ {
                N: self.N.clone(),
            }));
        checked_accessor_const_unsigned_int_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_const_unsigned_int_ {
    fn default() -> Self {
        { checked_accessor_const_unsigned_int_::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_const_float_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_float_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_const_float_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
    pub fn new_3(acc: Ptr<checked_accessor_float_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_const_float_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_const_float_> =
            Rc::new(RefCell::new(checked_accessor_const_float_ {
                N: self.N.clone(),
            }));
        checked_accessor_const_float_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_const_float_ {
    fn default() -> Self {
        { checked_accessor_const_float_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_float_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_float_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_const_double_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_double_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_const_double_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_const_double_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_const_double_> =
            Rc::new(RefCell::new(checked_accessor_const_double_ {
                N: self.N.clone(),
            }));
        checked_accessor_const_double_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_const_double_ {
    fn default() -> Self {
        { checked_accessor_const_double_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_MinimalElementType_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_MinimalElementType_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_const_MinimalElementType_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_MinimalElementType_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
const _: () = assert!(
    true,
    "std::is_constructible_v<checked_accessor<const int>, const checked_accessor<int>&>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<const checked_accessor<int>&, checked_accessor<const int>>"
);
pub fn test_mdspan_types_278(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_279(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_280(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_281(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_282(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_283(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_284(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_285(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_286(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_287(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_288(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_289(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_290(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_291(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_292(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_293(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_294(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_295(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_296(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_297(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_298(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_299(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_300(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_301(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_302(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_303(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_304(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_305(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_306(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_307(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_308(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_309(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_310(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_311(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_312(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_313(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_314(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_315(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_316(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_317(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_318(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_319(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_320(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_321(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_322(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_323(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_324(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_325(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_326(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_327(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_328(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_329(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_330(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_331(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_332(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_333(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_334(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_335(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_336(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_337(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_338(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_339(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_340(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_341(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_342(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_343(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_344(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_345(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_346(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_347(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_348(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_349(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_350(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_351(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_352(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_353(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_354(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_355(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_356(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_357(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_358(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_359(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_360(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_361(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_362(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_363(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_364(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_365(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_366(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_367(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_368(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_369(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_370(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_371(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_372(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_373(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_374(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_375(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_376(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_377(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_378(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_379(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_380(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_381(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_382(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_383(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_384(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_385(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_386(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_387(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_388(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_389(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_390(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_391(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_392(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_393(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_394(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_395(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_396(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_397(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_398(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_399(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_400(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_401(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_402(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_403(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_404(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_405(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_406(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_407(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_408(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_409(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_410(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_411(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_412(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_413(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_414(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_415(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_416(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_417(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_418(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_419(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_420(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_421(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_422(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_423(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_424(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_425(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_426(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_427(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_428(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_429(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_430(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_431(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_432(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_433(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_434(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_435(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_436(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_437(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_438(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_439(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_440(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_441(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_442(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_443(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_444(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_445(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_446(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_447(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_448(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_449(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_450(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_451(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_452(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_453(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_454(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_455(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_456(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_457(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_458(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_459(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_460(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_461(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_462(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_463(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_464(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_465(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_466(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_467(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_468(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_469(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {
            const _: () = assert!(true, "");;
        }
    }
}
pub fn test_mdspan_types_470(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_471(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_472(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_473(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_474(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_475(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(true, "");;
    {
        const _: () = assert!(true, "");;
        {}
    }
}
pub fn test_mdspan_types_476(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_477(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_478(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_479(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_480(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_481(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_482(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_483(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_484(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_485(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_486(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_487(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_488(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_489(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_490(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_491(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_492(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn test_mdspan_types_493(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "");;
    {}
}
pub fn mixin_extents_494(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_278(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_279(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_280(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_281(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_282(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_283(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_496(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_284(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_285(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_286(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_287(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_288(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_289(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_497(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_290(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_291(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_292(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_293(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_294(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_295(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_498(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_296(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_297(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_298(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_299(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_300(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_301(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_499(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_302(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_303(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_304(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_305(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_306(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_307(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_500(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_308(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_309(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_310(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_311(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_312(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_313(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_501(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_314(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_315(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_316(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_317(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_318(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_319(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_502(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_320(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_321(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_322(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_323(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_324(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_325(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_503(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_326(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_327(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_328(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_329(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_330(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_331(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_504(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_332(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_333(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_334(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_335(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_336(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_337(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_505(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_338(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_339(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_340(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_341(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_342(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_343(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_506(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_344(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_345(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_346(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_347(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_348(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_349(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_507(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_350(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_351(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_352(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_353(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_354(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_355(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_508(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_356(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_357(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_358(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_359(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_360(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_361(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_509(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_362(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_363(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_364(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_365(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_366(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_367(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_510(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_368(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_369(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_370(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_371(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_372(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_373(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_511(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_374(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_375(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_376(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_377(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_378(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_379(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_512(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_380(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_381(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_382(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_383(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_384(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_385(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_513(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_386(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_387(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_388(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_389(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_390(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_391(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_514(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_392(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_393(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_394(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_395(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_396(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_397(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_515(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_398(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_399(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_400(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_401(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_402(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_403(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_516(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_404(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_405(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_406(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_407(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_408(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_409(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_517(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_410(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_411(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_412(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_413(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_414(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_415(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_518(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_416(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_417(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_418(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_419(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_420(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_421(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_519(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_422(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_423(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_424(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_425(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_426(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_427(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_520(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_428(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_429(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_430(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_431(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_432(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_433(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_521(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_434(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_435(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_436(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_437(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_438(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_439(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_522(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_440(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_441(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_442(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_443(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_444(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_445(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_523(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_446(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_447(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_448(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_449(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_450(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_451(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_524(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_452(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_453(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_454(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_455(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_456(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_457(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_525(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_458(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_459(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_460(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_461(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_462(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_463(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_526(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_464(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_465(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_466(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_467(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_468(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_469(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_527(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_470(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_471(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_472(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_473(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_474(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_475(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_528(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_260(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_476(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_261(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_477(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_262(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_478(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_263(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_479(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_264(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_480(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_265(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_481(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_529(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_266(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_482(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_267(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_483(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_268(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_484(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_269(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_485(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_270(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_486(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_271(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_487(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_530(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_495.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_272((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_488(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_273((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_489(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_274((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_490(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_275 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_491(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_276 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_492(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_277 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_493(_handle, _map.as_pointer(), _acc)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct SizeTIntType {
    #[offset(0)]
    pub val: (),
}
impl SizeTIntType {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
pub fn test_no_layout_deduction_guides_531(handle: Ptr<Ptr<()>>, _a1: Ptr<()>) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
pub fn test_no_layout_deduction_guides_532(handle: Ptr<Ptr<()>>, _a1: Ptr<()>) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
pub fn test_no_layout_deduction_guides_533(handle: Ptr<Ptr<()>>, _a1: Ptr<()>) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
pub fn test_no_layout_deduction_guides_534(handle: Ptr<Ptr<()>>, _a1: Ptr<()>) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
pub fn test_no_layout_deduction_guides_535(
    handle: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<()>,
) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
pub fn test_no_layout_deduction_guides_536(
    handle: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<()>,
) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
pub fn test_no_layout_deduction_guides_537(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<checked_accessor_int_>,
) -> () {
    return false;
}
pub fn test_no_layout_deduction_guides_538(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<checked_accessor_const_int_>,
) -> () {
    return false;
}
pub fn test_no_layout_deduction_guides_539(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<checked_accessor_double_>,
) -> () {
    return false;
}
pub fn test_no_layout_deduction_guides_540(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<checked_accessor_const_double_>,
) -> () {
    return false;
}
pub fn test_no_layout_deduction_guides_541(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<checked_accessor_MinimalElementType_>,
) -> () {
    return false;
}
pub fn test_no_layout_deduction_guides_542(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<checked_accessor_const_MinimalElementType_>,
) -> () {
    return false;
}
pub fn mixin_layout_543(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_494(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_496(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_497(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_544(handle: Ptr<move_counted_handle_int_>, acc: Ptr<checked_accessor_int_>) {
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_498(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_499(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_500(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_545(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_501(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_502(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_503(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_546(
    handle: Ptr<move_counted_handle_const_int_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_504(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_505(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_506(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_547(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_507(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_508(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_509(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_548(
    handle: Ptr<no_default_ctor_handle_double_>,
    acc: Ptr<checked_accessor_double_>,
) {
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_510(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_511(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_512(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_549(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_513(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_514(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_515(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_550(
    handle: Ptr<move_counted_handle_const_double_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_516(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_517(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_518(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_551(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_519(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_520(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_521(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_552(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_522(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_523(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_524(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_553(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_525(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_526(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_527(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_layout_554(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_528(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_529(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_530(_handle, _layout.as_pointer(), _acc)
    });
    unimplemented!();
}
pub fn mixin_accessor_555() {
    let elements: Value<ElementPool_int__1024_> =
        Rc::new(RefCell::new(ElementPool_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_543(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_int_> =
        Rc::new(RefCell::new(checked_accessor_int_::new({ 1024_() })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_int_> =
            Rc::new(RefCell::new(move_counted_handle_int_::new_1({
                ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_544(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_556() {
    let elements: Value<ElementPool_const_int__1024_> =
        Rc::new(RefCell::new(ElementPool_const_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_545(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_const_int_> =
        Rc::new(RefCell::new(checked_accessor_const_int_::new({ 1024_() })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_int_> =
            Rc::new(RefCell::new(move_counted_handle_const_int_::new_1({
                ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_546(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_557() {
    let elements: Value<ElementPool_double__1024_> =
        Rc::new(RefCell::new(ElementPool_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_547(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_double_> =
        Rc::new(RefCell::new(checked_accessor_double_::new({ 1024_() })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<no_default_ctor_handle_double_> =
            Rc::new(RefCell::new(no_default_ctor_handle_double_::new_1({
                ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_548(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_558() {
    let elements: Value<ElementPool_const_double__1024_> =
        Rc::new(RefCell::new(ElementPool_const_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_549(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_const_double_> =
        Rc::new(RefCell::new(checked_accessor_const_double_::new_2({
            1024_()
        })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_double_> =
            Rc::new(RefCell::new(move_counted_handle_const_double_::new_1({
                ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_550(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_559() {
    let elements: Value<ElementPool_MinimalElementType__1024_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_551(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_MinimalElementType_> =
        Rc::new(RefCell::new(checked_accessor_MinimalElementType_::new({
            1024_()
        })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_MinimalElementType_> = Rc::new(RefCell::new(
            move_counted_handle_MinimalElementType_::new_1({
                ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) })
            }),
        ));
        mixin_layout_552(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_560() {
    let elements: Value<ElementPool_const_MinimalElementType__1024_> = Rc::new(RefCell::new(
        ElementPool_const_MinimalElementType__1024_::new(),
    ));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_553(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_const_MinimalElementType_> = Rc::new(RefCell::new(
        checked_accessor_const_MinimalElementType_::new({ 1024_() }),
    ));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_MinimalElementType_> = Rc::new(RefCell::new(
            move_counted_handle_const_MinimalElementType_::new_1({
                ({
                    ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer())
                })
            }),
        ));
        mixin_layout_554(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn test_561() -> () {
    ({ mixin_accessor_555() });
    ({ mixin_accessor_556() });
    ({ mixin_accessor_557() });
    ({ mixin_accessor_558() });
    ({ mixin_accessor_559() });
    ({ mixin_accessor_560() });
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..12).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    const _: () = assert!(true, "");;
    let mut p: Ptr<()> = (a.as_pointer() as Ptr<()>);
    const _: () = assert!(true, "");;
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
    ({ test_561() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct move_counted_handle_float_;
pub trait ElementPool_MinimalElementType__1024_Impl {
    fn get_ptr(&self) -> Ptr<MinimalElementType>;
    fn destructor(&self);
}
impl ElementPool_MinimalElementType__1024_Impl for Ptr<ElementPool_MinimalElementType__1024_> {
    fn get_ptr(&self) -> Ptr<MinimalElementType> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_const_MinimalElementType__1024_Impl {
    fn get_ptr(&self) -> Ptr<MinimalElementType>;
    fn destructor(&self);
}
impl ElementPool_const_MinimalElementType__1024_Impl
    for Ptr<ElementPool_const_MinimalElementType__1024_>
{
    fn get_ptr(&self) -> Ptr<MinimalElementType> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_const_double__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_const_double__1024_Impl for Ptr<ElementPool_const_double__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_const_int__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_const_int__1024_Impl for Ptr<ElementPool_const_int__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_double__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_double__1024_Impl for Ptr<ElementPool_double__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_int__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_int__1024_Impl for Ptr<ElementPool_int__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait SizeTIntTypeImpl {
    fn to_(&self) -> ();
}
impl SizeTIntTypeImpl for Ptr<SizeTIntType> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait checked_accessor_MinimalElementType_Impl {
    fn access(
        &self,
        mut p: move_counted_handle_MinimalElementType_,
        mut i: (),
    ) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
    fn offset(
        &self,
        mut p: move_counted_handle_MinimalElementType_,
        mut i: (),
    ) -> move_counted_handle_MinimalElementType_ {
        unimplemented!()
    }
}
impl checked_accessor_MinimalElementType_Impl for Ptr<checked_accessor_MinimalElementType_> {}
pub trait checked_accessor_const_MinimalElementType_Impl {
    fn access(
        &self,
        mut p: move_counted_handle_const_MinimalElementType_,
        mut i: (),
    ) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_MinimalElementType_,
        mut i: (),
    ) -> move_counted_handle_const_MinimalElementType_ {
        unimplemented!()
    }
}
impl checked_accessor_const_MinimalElementType_Impl
    for Ptr<checked_accessor_const_MinimalElementType_>
{
}
pub trait checked_accessor_const_double_Impl {
    fn access(&self, p: move_counted_handle_const_double_, i: ()) -> Ptr<()>;
    fn offset(
        &self,
        p: move_counted_handle_const_double_,
        i: (),
    ) -> move_counted_handle_const_double_;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_double_>,
    ) -> Ptr<checked_accessor_const_double_>;
}
impl checked_accessor_const_double_Impl for Ptr<checked_accessor_const_double_> {
    fn access(&self, p: move_counted_handle_const_double_, mut i: ()) -> Ptr<()> {
        let p: Value<move_counted_handle_const_double_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return ({ move_counted_handle_const_double_Impl::operator_index(&p.as_pointer(), i) });
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_double_,
        mut i: (),
    ) -> move_counted_handle_const_double_ {
        unimplemented!();
        return move_counted_handle_const_double_::new_1({ p.ptr.offset((i) as isize) });
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_double_>,
    ) -> Ptr<checked_accessor_const_double_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait checked_accessor_const_float_Impl {
    fn access(&self, p: move_counted_handle_const_float_, i: ()) -> Ptr<()>;
    fn offset(
        &self,
        p: move_counted_handle_const_float_,
        i: (),
    ) -> move_counted_handle_const_float_;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_float_>,
    ) -> Ptr<checked_accessor_const_float_>;
}
impl checked_accessor_const_float_Impl for Ptr<checked_accessor_const_float_> {
    fn access(&self, p: move_counted_handle_const_float_, mut i: ()) -> Ptr<()> {
        let p: Value<move_counted_handle_const_float_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return ({ move_counted_handle_const_float_Impl::operator_index(&p.as_pointer(), i) });
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_float_,
        mut i: (),
    ) -> move_counted_handle_const_float_ {
        unimplemented!();
        return move_counted_handle_const_float_::new_1({ p.ptr.offset((i) as isize) });
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_float_>,
    ) -> Ptr<checked_accessor_const_float_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait checked_accessor_const_int_Impl {
    fn access(&self, mut p: move_counted_handle_const_int_, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_int_,
        mut i: (),
    ) -> move_counted_handle_const_int_ {
        unimplemented!()
    }
}
impl checked_accessor_const_int_Impl for Ptr<checked_accessor_const_int_> {}
pub trait checked_accessor_const_unsigned_int_Impl {
    fn access(&self, p: not_const_convertible_handle_const_unsigned_int_, i: ()) -> ();
    fn offset(&self, p: not_const_convertible_handle_const_unsigned_int_, i: ()) -> Ptr<()>;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_unsigned_int_>,
    ) -> Ptr<checked_accessor_const_unsigned_int_>;
}
impl checked_accessor_const_unsigned_int_Impl for Ptr<checked_accessor_const_unsigned_int_> {
    fn access(&self, p: not_const_convertible_handle_const_unsigned_int_, mut i: ()) -> () {
        let p: Value<not_const_convertible_handle_const_unsigned_int_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return (({
            not_const_convertible_handle_const_unsigned_int_Impl::operator_index(&p.as_pointer(), i)
        })
        .read());
    }
    fn offset(
        &self,
        mut p: not_const_convertible_handle_const_unsigned_int_,
        mut i: (),
    ) -> Ptr<()> {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_unsigned_int_>,
    ) -> Ptr<checked_accessor_const_unsigned_int_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait checked_accessor_double_Impl {
    fn access(&self, p: no_default_ctor_handle_double_, i: ()) -> Ptr<()>;
    fn offset(&self, p: no_default_ctor_handle_double_, i: ()) -> no_default_ctor_handle_double_;
}
impl checked_accessor_double_Impl for Ptr<checked_accessor_double_> {
    fn access(&self, mut p: no_default_ctor_handle_double_, mut i: ()) -> Ptr<()> {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    fn offset(
        &self,
        mut p: no_default_ctor_handle_double_,
        mut i: (),
    ) -> no_default_ctor_handle_double_ {
        unimplemented!();
        return no_default_ctor_handle_double_::new_1({ p.ptr.offset((i) as isize) });
    }
}
pub trait checked_accessor_float_Impl {
    fn access(&self, mut p: move_counted_handle_float_, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn offset(&self, mut p: move_counted_handle_float_, mut i: ()) -> move_counted_handle_float_ {
        unimplemented!()
    }
}
impl checked_accessor_float_Impl for Ptr<checked_accessor_float_> {}
pub trait checked_accessor_int_Impl {
    fn access(&self, mut p: move_counted_handle_int_, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn offset(&self, mut p: move_counted_handle_int_, mut i: ()) -> move_counted_handle_int_ {
        unimplemented!()
    }
}
impl checked_accessor_int_Impl for Ptr<checked_accessor_int_> {}
pub trait checked_accessor_unsigned_int_Impl {
    fn access(&self, p: not_const_convertible_handle_unsigned_int_, i: ()) -> ();
    fn offset(&self, p: not_const_convertible_handle_unsigned_int_, i: ()) -> Ptr<()>;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_unsigned_int_>,
    ) -> Ptr<checked_accessor_unsigned_int_>;
}
impl checked_accessor_unsigned_int_Impl for Ptr<checked_accessor_unsigned_int_> {
    fn access(&self, p: not_const_convertible_handle_unsigned_int_, mut i: ()) -> () {
        let p: Value<not_const_convertible_handle_unsigned_int_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return (({
            not_const_convertible_handle_unsigned_int_Impl::operator_index(&p.as_pointer(), i)
        })
        .read());
    }
    fn offset(&self, mut p: not_const_convertible_handle_unsigned_int_, mut i: ()) -> Ptr<()> {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_unsigned_int_>,
    ) -> Ptr<checked_accessor_unsigned_int_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_int__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_int__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_int__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>
{
}
pub trait layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__> {
}
pub trait layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        >,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>
{
}
pub trait layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__> {
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>
{
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__> {
}
pub trait move_counted_handle_MinimalElementType_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
}
impl move_counted_handle_MinimalElementType_Impl for Ptr<move_counted_handle_MinimalElementType_> {}
pub trait move_counted_handle_const_MinimalElementType_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
}
impl move_counted_handle_const_MinimalElementType_Impl
    for Ptr<move_counted_handle_const_MinimalElementType_>
{
}
pub trait move_counted_handle_const_double_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl move_counted_handle_const_double_Impl for Ptr<move_counted_handle_const_double_> {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub trait move_counted_handle_const_float_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl move_counted_handle_const_float_Impl for Ptr<move_counted_handle_const_float_> {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub trait move_counted_handle_const_int_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl move_counted_handle_const_int_Impl for Ptr<move_counted_handle_const_int_> {}
pub trait move_counted_handle_int_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl move_counted_handle_int_Impl for Ptr<move_counted_handle_int_> {}
pub trait not_const_convertible_handle_const_unsigned_int_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl not_const_convertible_handle_const_unsigned_int_Impl
    for Ptr<not_const_convertible_handle_const_unsigned_int_>
{
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub trait not_const_convertible_handle_unsigned_int_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl not_const_convertible_handle_unsigned_int_Impl
    for Ptr<not_const_convertible_handle_unsigned_int_>
{
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = Wrap_254.with(|_| ());
    let _ = Wrap_255.with(|_| ());
    let _ = Wrap_256.with(|_| ());
    let _ = Wrap_257.with(|_| ());
    let _ = Wrap_258.with(|_| ());
    let _ = Wrap_259.with(|_| ());
}
