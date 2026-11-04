extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_imp_0(_a0: std_integral_constant_bool__true_) {
    let _a0: Value<std_integral_constant_bool__true_> = Rc::new(RefCell::new(_a0));
    assert!(({ isnan_1(({ std_numeric_limits_float_::signaling_NaN() }),) }));
    assert!(({ isnan_1(({ std_numeric_limits_float_::signaling_NaN() }),) }));
    assert!(({ isnan_1(({ std_numeric_limits_float_::signaling_NaN() }),) }));
    assert!(({ isnan_1(({ std_numeric_limits_float_::signaling_NaN() }),) }));
}
pub fn test_imp_2(_a0: std_integral_constant_bool__true_) {
    let _a0: Value<std_integral_constant_bool__true_> = Rc::new(RefCell::new(_a0));
    assert!(({ isnan_3(({ std_numeric_limits_double_::signaling_NaN() }),) }));
    assert!(({ isnan_3(({ std_numeric_limits_double_::signaling_NaN() }),) }));
    assert!(({ isnan_3(({ std_numeric_limits_double_::signaling_NaN() }),) }));
    assert!(({ isnan_3(({ std_numeric_limits_double_::signaling_NaN() }),) }));
}
pub fn test_imp_4(_a0: std_integral_constant_bool__true_) {
    let _a0: Value<std_integral_constant_bool__true_> = Rc::new(RefCell::new(_a0));
    assert!(({ isnan_5(({ std_numeric_limits_long_double_::signaling_NaN() }),) }));
    assert!(({ isnan_5(({ std_numeric_limits_long_double_::signaling_NaN() }),) }));
    assert!(({ isnan_5(({ std_numeric_limits_long_double_::signaling_NaN() }),) }));
    assert!(({ isnan_5(({ std_numeric_limits_long_double_::signaling_NaN() }),) }));
}
pub fn test_imp_6(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_bool_::signaling_NaN() }) as i32) == (false as i32)));
    assert!(((({ std_numeric_limits_bool_::signaling_NaN() }) as i32) == (false as i32)));
    assert!(((({ std_numeric_limits_bool_::signaling_NaN() }) as i32) == (false as i32)));
    assert!(((({ std_numeric_limits_bool_::signaling_NaN() }) as i32) == (false as i32)));
}
pub fn test_imp_7(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
    assert!(((({ std_numeric_limits_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
    assert!(((({ std_numeric_limits_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
    assert!(((({ std_numeric_limits_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
}
pub fn test_imp_8(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_signed_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
    assert!(((({ std_numeric_limits_signed_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
    assert!(((({ std_numeric_limits_signed_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
    assert!(((({ std_numeric_limits_signed_char_::signaling_NaN() }) as i32) == (0_i8 as i32)));
}
pub fn test_imp_9(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_unsigned_char_::signaling_NaN() }) as i32) == (0_u8 as i32)));
    assert!(((({ std_numeric_limits_unsigned_char_::signaling_NaN() }) as i32) == (0_u8 as i32)));
    assert!(((({ std_numeric_limits_unsigned_char_::signaling_NaN() }) as i32) == (0_u8 as i32)));
    assert!(((({ std_numeric_limits_unsigned_char_::signaling_NaN() }) as i32) == (0_u8 as i32)));
}
pub fn test_imp_10(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_wchar_t_::signaling_NaN() }) == 0_i32));
    assert!((({ std_numeric_limits_wchar_t_::signaling_NaN() }) == 0_i32));
    assert!((({ std_numeric_limits_wchar_t_::signaling_NaN() }) == 0_i32));
    assert!((({ std_numeric_limits_wchar_t_::signaling_NaN() }) == 0_i32));
}
pub fn test_imp_11(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_char8_t_::signaling_NaN() }) as i32) == (0_u8 as i32)));
    assert!(((({ std_numeric_limits_char8_t_::signaling_NaN() }) as i32) == (0_u8 as i32)));
    assert!(((({ std_numeric_limits_char8_t_::signaling_NaN() }) as i32) == (0_u8 as i32)));
    assert!(((({ std_numeric_limits_char8_t_::signaling_NaN() }) as i32) == (0_u8 as i32)));
}
pub fn test_imp_12(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_char16_t_::signaling_NaN() }) as i32) == (0_u16 as i32)));
    assert!(((({ std_numeric_limits_char16_t_::signaling_NaN() }) as i32) == (0_u16 as i32)));
    assert!(((({ std_numeric_limits_char16_t_::signaling_NaN() }) as i32) == (0_u16 as i32)));
    assert!(((({ std_numeric_limits_char16_t_::signaling_NaN() }) as i32) == (0_u16 as i32)));
}
pub fn test_imp_13(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_char32_t_::signaling_NaN() }) == 0_u32));
    assert!((({ std_numeric_limits_char32_t_::signaling_NaN() }) == 0_u32));
    assert!((({ std_numeric_limits_char32_t_::signaling_NaN() }) == 0_u32));
    assert!((({ std_numeric_limits_char32_t_::signaling_NaN() }) == 0_u32));
}
pub fn test_imp_14(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_short_::signaling_NaN() }) as i32) == (0_i16 as i32)));
    assert!(((({ std_numeric_limits_short_::signaling_NaN() }) as i32) == (0_i16 as i32)));
    assert!(((({ std_numeric_limits_short_::signaling_NaN() }) as i32) == (0_i16 as i32)));
    assert!(((({ std_numeric_limits_short_::signaling_NaN() }) as i32) == (0_i16 as i32)));
}
pub fn test_imp_15(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!(((({ std_numeric_limits_unsigned_short_::signaling_NaN() }) as i32) == (0_u16 as i32)));
    assert!(((({ std_numeric_limits_unsigned_short_::signaling_NaN() }) as i32) == (0_u16 as i32)));
    assert!(((({ std_numeric_limits_unsigned_short_::signaling_NaN() }) as i32) == (0_u16 as i32)));
    assert!(((({ std_numeric_limits_unsigned_short_::signaling_NaN() }) as i32) == (0_u16 as i32)));
}
pub fn test_imp_16(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_int_::signaling_NaN() }) == 0_i32));
    assert!((({ std_numeric_limits_int_::signaling_NaN() }) == 0_i32));
    assert!((({ std_numeric_limits_int_::signaling_NaN() }) == 0_i32));
    assert!((({ std_numeric_limits_int_::signaling_NaN() }) == 0_i32));
}
pub fn test_imp_17(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_unsigned_int_::signaling_NaN() }) == 0_u32));
    assert!((({ std_numeric_limits_unsigned_int_::signaling_NaN() }) == 0_u32));
    assert!((({ std_numeric_limits_unsigned_int_::signaling_NaN() }) == 0_u32));
    assert!((({ std_numeric_limits_unsigned_int_::signaling_NaN() }) == 0_u32));
}
pub fn test_imp_18(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_long_::signaling_NaN() }) == 0_i64));
    assert!((({ std_numeric_limits_long_::signaling_NaN() }) == 0_i64));
    assert!((({ std_numeric_limits_long_::signaling_NaN() }) == 0_i64));
    assert!((({ std_numeric_limits_long_::signaling_NaN() }) == 0_i64));
}
pub fn test_imp_19(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_unsigned_long_::signaling_NaN() }) == 0_u64));
    assert!((({ std_numeric_limits_unsigned_long_::signaling_NaN() }) == 0_u64));
    assert!((({ std_numeric_limits_unsigned_long_::signaling_NaN() }) == 0_u64));
    assert!((({ std_numeric_limits_unsigned_long_::signaling_NaN() }) == 0_u64));
}
pub fn test_imp_20(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_long_long_::signaling_NaN() }) == 0_i64));
    assert!((({ std_numeric_limits_long_long_::signaling_NaN() }) == 0_i64));
    assert!((({ std_numeric_limits_long_long_::signaling_NaN() }) == 0_i64));
    assert!((({ std_numeric_limits_long_long_::signaling_NaN() }) == 0_i64));
}
pub fn test_imp_21(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_unsigned_long_long_::signaling_NaN() }) == 0_u64));
    assert!((({ std_numeric_limits_unsigned_long_long_::signaling_NaN() }) == 0_u64));
    assert!((({ std_numeric_limits_unsigned_long_long_::signaling_NaN() }) == 0_u64));
    assert!((({ std_numeric_limits_unsigned_long_long_::signaling_NaN() }) == 0_u64));
}
pub fn test_imp_22(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits___int128_::signaling_NaN() }) == 0_i128));
    assert!((({ std_numeric_limits___int128_::signaling_NaN() }) == 0_i128));
    assert!((({ std_numeric_limits___int128_::signaling_NaN() }) == 0_i128));
    assert!((({ std_numeric_limits___int128_::signaling_NaN() }) == 0_i128));
}
pub fn test_imp_23(_a0: std_integral_constant_bool__false_) {
    let _a0: Value<std_integral_constant_bool__false_> = Rc::new(RefCell::new(_a0));
    assert!((({ std_numeric_limits_unsigned___int128_::signaling_NaN() }) == 0_u128));
    assert!((({ std_numeric_limits_unsigned___int128_::signaling_NaN() }) == 0_u128));
    assert!((({ std_numeric_limits_unsigned___int128_::signaling_NaN() }) == 0_u128));
    assert!((({ std_numeric_limits_unsigned___int128_::signaling_NaN() }) == 0_u128));
}
pub fn test_24() {
    ({ test_imp_6(<std_is_floating_point_bool_>::default()) });
}
pub fn test_25() {
    ({ test_imp_7(<std_is_floating_point_char_>::default()) });
}
pub fn test_26() {
    ({ test_imp_8(<std_is_floating_point_signed_char_>::default()) });
}
pub fn test_27() {
    ({ test_imp_9(<std_is_floating_point_unsigned_char_>::default()) });
}
pub fn test_28() {
    ({ test_imp_10(<std_is_floating_point_wchar_t_>::default()) });
}
pub fn test_29() {
    ({ test_imp_11(<std_is_floating_point_char8_t_>::default()) });
}
pub fn test_30() {
    ({ test_imp_12(<std_is_floating_point_char16_t_>::default()) });
}
pub fn test_31() {
    ({ test_imp_13(<std_is_floating_point_char32_t_>::default()) });
}
pub fn test_32() {
    ({ test_imp_14(<std_is_floating_point_short_>::default()) });
}
pub fn test_33() {
    ({ test_imp_15(<std_is_floating_point_unsigned_short_>::default()) });
}
pub fn test_34() {
    ({ test_imp_16(<std_is_floating_point_int_>::default()) });
}
pub fn test_35() {
    ({ test_imp_17(<std_is_floating_point_unsigned_int_>::default()) });
}
pub fn test_36() {
    ({ test_imp_18(<std_is_floating_point_long_>::default()) });
}
pub fn test_37() {
    ({ test_imp_19(<std_is_floating_point_unsigned_long_>::default()) });
}
pub fn test_38() {
    ({ test_imp_20(<std_is_floating_point_long_long_>::default()) });
}
pub fn test_39() {
    ({ test_imp_21(<std_is_floating_point_unsigned_long_long_>::default()) });
}
pub fn test_40() {
    ({ test_imp_22(<std_is_floating_point___int128_>::default()) });
}
pub fn test_41() {
    ({ test_imp_23(<std_is_floating_point_unsigned___int128_>::default()) });
}
pub fn test_42() {
    ({ test_imp_0(<std_is_floating_point_float_>::default()) });
}
pub fn test_43() {
    ({ test_imp_2(<std_is_floating_point_double_>::default()) });
}
pub fn test_44() {
    ({ test_imp_4(<std_is_floating_point_long_double_>::default()) });
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
