extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut expected: bool) {
    assert!(((false as i32) == (expected as i32)));
    assert!((is_bounded_1.with(|rc| *rc.borrow())) || (!(is_signed_2.with(|rc| *rc.borrow()))));
    assert!(((false as i32) == (expected as i32)));
    assert!((is_bounded_1.with(|rc| *rc.borrow())) || (!(is_signed_2.with(|rc| *rc.borrow()))));
    assert!(((false as i32) == (expected as i32)));
    assert!((is_bounded_1.with(|rc| *rc.borrow())) || (!(is_signed_2.with(|rc| *rc.borrow()))));
    assert!(((false as i32) == (expected as i32)));
    assert!((is_bounded_1.with(|rc| *rc.borrow())) || (!(is_signed_2.with(|rc| *rc.borrow()))));
}
pub fn test_3(mut expected: i8) {
    assert!(((libc::c_char::MIN as i32) == (expected as i32)));
    assert!((is_bounded_4.with(|rc| *rc.borrow())) || (!(is_signed_5.with(|rc| *rc.borrow()))));
    assert!(((libc::c_char::MIN as i32) == (expected as i32)));
    assert!((is_bounded_4.with(|rc| *rc.borrow())) || (!(is_signed_5.with(|rc| *rc.borrow()))));
    assert!(((libc::c_char::MIN as i32) == (expected as i32)));
    assert!((is_bounded_4.with(|rc| *rc.borrow())) || (!(is_signed_5.with(|rc| *rc.borrow()))));
    assert!(((libc::c_char::MIN as i32) == (expected as i32)));
    assert!((is_bounded_4.with(|rc| *rc.borrow())) || (!(is_signed_5.with(|rc| *rc.borrow()))));
}
pub fn test_6(mut expected: i8) {
    assert!(((i8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_7.with(|rc| *rc.borrow())) || (!(is_signed_8.with(|rc| *rc.borrow()))));
    assert!(((i8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_7.with(|rc| *rc.borrow())) || (!(is_signed_8.with(|rc| *rc.borrow()))));
    assert!(((i8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_7.with(|rc| *rc.borrow())) || (!(is_signed_8.with(|rc| *rc.borrow()))));
    assert!(((i8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_7.with(|rc| *rc.borrow())) || (!(is_signed_8.with(|rc| *rc.borrow()))));
}
pub fn test_9(mut expected: u8) {
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_10.with(|rc| *rc.borrow())) || (!(is_signed_11.with(|rc| *rc.borrow()))));
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_10.with(|rc| *rc.borrow())) || (!(is_signed_11.with(|rc| *rc.borrow()))));
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_10.with(|rc| *rc.borrow())) || (!(is_signed_11.with(|rc| *rc.borrow()))));
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_10.with(|rc| *rc.borrow())) || (!(is_signed_11.with(|rc| *rc.borrow()))));
}
pub fn test_12(mut expected: i32) {
    assert!((i32::MIN == expected));
    assert!((is_bounded_13.with(|rc| *rc.borrow())) || (!(is_signed_14.with(|rc| *rc.borrow()))));
    assert!((i32::MIN == expected));
    assert!((is_bounded_13.with(|rc| *rc.borrow())) || (!(is_signed_14.with(|rc| *rc.borrow()))));
    assert!((i32::MIN == expected));
    assert!((is_bounded_13.with(|rc| *rc.borrow())) || (!(is_signed_14.with(|rc| *rc.borrow()))));
    assert!((i32::MIN == expected));
    assert!((is_bounded_13.with(|rc| *rc.borrow())) || (!(is_signed_14.with(|rc| *rc.borrow()))));
}
pub fn test_15(mut expected: u8) {
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_16.with(|rc| *rc.borrow())) || (!(is_signed_17.with(|rc| *rc.borrow()))));
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_16.with(|rc| *rc.borrow())) || (!(is_signed_17.with(|rc| *rc.borrow()))));
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_16.with(|rc| *rc.borrow())) || (!(is_signed_17.with(|rc| *rc.borrow()))));
    assert!(((u8::MIN as i32) == (expected as i32)));
    assert!((is_bounded_16.with(|rc| *rc.borrow())) || (!(is_signed_17.with(|rc| *rc.borrow()))));
}
pub fn test_18(mut expected: u16) {
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_19.with(|rc| *rc.borrow())) || (!(is_signed_20.with(|rc| *rc.borrow()))));
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_19.with(|rc| *rc.borrow())) || (!(is_signed_20.with(|rc| *rc.borrow()))));
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_19.with(|rc| *rc.borrow())) || (!(is_signed_20.with(|rc| *rc.borrow()))));
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_19.with(|rc| *rc.borrow())) || (!(is_signed_20.with(|rc| *rc.borrow()))));
}
pub fn test_21(mut expected: u32) {
    assert!((u32::MIN == expected));
    assert!((is_bounded_22.with(|rc| *rc.borrow())) || (!(is_signed_23.with(|rc| *rc.borrow()))));
    assert!((u32::MIN == expected));
    assert!((is_bounded_22.with(|rc| *rc.borrow())) || (!(is_signed_23.with(|rc| *rc.borrow()))));
    assert!((u32::MIN == expected));
    assert!((is_bounded_22.with(|rc| *rc.borrow())) || (!(is_signed_23.with(|rc| *rc.borrow()))));
    assert!((u32::MIN == expected));
    assert!((is_bounded_22.with(|rc| *rc.borrow())) || (!(is_signed_23.with(|rc| *rc.borrow()))));
}
pub fn test_24(mut expected: i16) {
    assert!(((i16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_25.with(|rc| *rc.borrow())) || (!(is_signed_26.with(|rc| *rc.borrow()))));
    assert!(((i16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_25.with(|rc| *rc.borrow())) || (!(is_signed_26.with(|rc| *rc.borrow()))));
    assert!(((i16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_25.with(|rc| *rc.borrow())) || (!(is_signed_26.with(|rc| *rc.borrow()))));
    assert!(((i16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_25.with(|rc| *rc.borrow())) || (!(is_signed_26.with(|rc| *rc.borrow()))));
}
pub fn test_27(mut expected: u16) {
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_28.with(|rc| *rc.borrow())) || (!(is_signed_29.with(|rc| *rc.borrow()))));
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_28.with(|rc| *rc.borrow())) || (!(is_signed_29.with(|rc| *rc.borrow()))));
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_28.with(|rc| *rc.borrow())) || (!(is_signed_29.with(|rc| *rc.borrow()))));
    assert!(((u16::MIN as i32) == (expected as i32)));
    assert!((is_bounded_28.with(|rc| *rc.borrow())) || (!(is_signed_29.with(|rc| *rc.borrow()))));
}
pub fn test_30(mut expected: i32) {
    assert!((i32::MIN == expected));
    assert!((is_bounded_31.with(|rc| *rc.borrow())) || (!(is_signed_32.with(|rc| *rc.borrow()))));
    assert!((i32::MIN == expected));
    assert!((is_bounded_31.with(|rc| *rc.borrow())) || (!(is_signed_32.with(|rc| *rc.borrow()))));
    assert!((i32::MIN == expected));
    assert!((is_bounded_31.with(|rc| *rc.borrow())) || (!(is_signed_32.with(|rc| *rc.borrow()))));
    assert!((i32::MIN == expected));
    assert!((is_bounded_31.with(|rc| *rc.borrow())) || (!(is_signed_32.with(|rc| *rc.borrow()))));
}
pub fn test_33(mut expected: u32) {
    assert!((u32::MIN == expected));
    assert!((is_bounded_34.with(|rc| *rc.borrow())) || (!(is_signed_35.with(|rc| *rc.borrow()))));
    assert!((u32::MIN == expected));
    assert!((is_bounded_34.with(|rc| *rc.borrow())) || (!(is_signed_35.with(|rc| *rc.borrow()))));
    assert!((u32::MIN == expected));
    assert!((is_bounded_34.with(|rc| *rc.borrow())) || (!(is_signed_35.with(|rc| *rc.borrow()))));
    assert!((u32::MIN == expected));
    assert!((is_bounded_34.with(|rc| *rc.borrow())) || (!(is_signed_35.with(|rc| *rc.borrow()))));
}
pub fn test_36(mut expected: i64) {
    assert!((i64::MIN == expected));
    assert!((is_bounded_37.with(|rc| *rc.borrow())) || (!(is_signed_38.with(|rc| *rc.borrow()))));
    assert!((i64::MIN == expected));
    assert!((is_bounded_37.with(|rc| *rc.borrow())) || (!(is_signed_38.with(|rc| *rc.borrow()))));
    assert!((i64::MIN == expected));
    assert!((is_bounded_37.with(|rc| *rc.borrow())) || (!(is_signed_38.with(|rc| *rc.borrow()))));
    assert!((i64::MIN == expected));
    assert!((is_bounded_37.with(|rc| *rc.borrow())) || (!(is_signed_38.with(|rc| *rc.borrow()))));
}
pub fn test_39(mut expected: u64) {
    assert!((u64::MIN == expected));
    assert!((is_bounded_40.with(|rc| *rc.borrow())) || (!(is_signed_41.with(|rc| *rc.borrow()))));
    assert!((u64::MIN == expected));
    assert!((is_bounded_40.with(|rc| *rc.borrow())) || (!(is_signed_41.with(|rc| *rc.borrow()))));
    assert!((u64::MIN == expected));
    assert!((is_bounded_40.with(|rc| *rc.borrow())) || (!(is_signed_41.with(|rc| *rc.borrow()))));
    assert!((u64::MIN == expected));
    assert!((is_bounded_40.with(|rc| *rc.borrow())) || (!(is_signed_41.with(|rc| *rc.borrow()))));
}
pub fn test_42(mut expected: i64) {
    assert!((i64::MIN == expected));
    assert!((is_bounded_43.with(|rc| *rc.borrow())) || (!(is_signed_44.with(|rc| *rc.borrow()))));
    assert!((i64::MIN == expected));
    assert!((is_bounded_43.with(|rc| *rc.borrow())) || (!(is_signed_44.with(|rc| *rc.borrow()))));
    assert!((i64::MIN == expected));
    assert!((is_bounded_43.with(|rc| *rc.borrow())) || (!(is_signed_44.with(|rc| *rc.borrow()))));
    assert!((i64::MIN == expected));
    assert!((is_bounded_43.with(|rc| *rc.borrow())) || (!(is_signed_44.with(|rc| *rc.borrow()))));
}
pub fn test_45(mut expected: u64) {
    assert!((({ std_numeric_limits_unsigned_long_long_::min() }) == expected));
    assert!((is_bounded_46.with(|rc| *rc.borrow())) || (!(is_signed_47.with(|rc| *rc.borrow()))));
    assert!((({ std_numeric_limits_unsigned_long_long_::min() }) == expected));
    assert!((is_bounded_46.with(|rc| *rc.borrow())) || (!(is_signed_47.with(|rc| *rc.borrow()))));
    assert!((({ std_numeric_limits_unsigned_long_long_::min() }) == expected));
    assert!((is_bounded_46.with(|rc| *rc.borrow())) || (!(is_signed_47.with(|rc| *rc.borrow()))));
    assert!((({ std_numeric_limits_unsigned_long_long_::min() }) == expected));
    assert!((is_bounded_46.with(|rc| *rc.borrow())) || (!(is_signed_47.with(|rc| *rc.borrow()))));
}
pub fn test_48(mut expected: i128) {
    assert!((i128::MIN == expected));
    assert!((is_bounded_49.with(|rc| *rc.borrow())) || (!(is_signed_50.with(|rc| *rc.borrow()))));
    assert!((i128::MIN == expected));
    assert!((is_bounded_49.with(|rc| *rc.borrow())) || (!(is_signed_50.with(|rc| *rc.borrow()))));
    assert!((i128::MIN == expected));
    assert!((is_bounded_49.with(|rc| *rc.borrow())) || (!(is_signed_50.with(|rc| *rc.borrow()))));
    assert!((i128::MIN == expected));
    assert!((is_bounded_49.with(|rc| *rc.borrow())) || (!(is_signed_50.with(|rc| *rc.borrow()))));
}
pub fn test_51(mut expected: u128) {
    assert!((({ std_numeric_limits_unsigned___int128_::min() }) == expected));
    assert!((is_bounded_52.with(|rc| *rc.borrow())) || (!(is_signed_53.with(|rc| *rc.borrow()))));
    assert!((({ std_numeric_limits_unsigned___int128_::min() }) == expected));
    assert!((is_bounded_52.with(|rc| *rc.borrow())) || (!(is_signed_53.with(|rc| *rc.borrow()))));
    assert!((({ std_numeric_limits_unsigned___int128_::min() }) == expected));
    assert!((is_bounded_52.with(|rc| *rc.borrow())) || (!(is_signed_53.with(|rc| *rc.borrow()))));
    assert!((({ std_numeric_limits_unsigned___int128_::min() }) == expected));
    assert!((is_bounded_52.with(|rc| *rc.borrow())) || (!(is_signed_53.with(|rc| *rc.borrow()))));
}
pub fn test_54(mut expected: f32) {
    assert!((f32::MIN_POSITIVE == expected));
    assert!((is_bounded_55.with(|rc| *rc.borrow())) || (!(is_signed_56.with(|rc| *rc.borrow()))));
    assert!((f32::MIN_POSITIVE == expected));
    assert!((is_bounded_55.with(|rc| *rc.borrow())) || (!(is_signed_56.with(|rc| *rc.borrow()))));
    assert!((f32::MIN_POSITIVE == expected));
    assert!((is_bounded_55.with(|rc| *rc.borrow())) || (!(is_signed_56.with(|rc| *rc.borrow()))));
    assert!((f32::MIN_POSITIVE == expected));
    assert!((is_bounded_55.with(|rc| *rc.borrow())) || (!(is_signed_56.with(|rc| *rc.borrow()))));
}
pub fn test_57(mut expected: f64) {
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_58.with(|rc| *rc.borrow())) || (!(is_signed_59.with(|rc| *rc.borrow()))));
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_58.with(|rc| *rc.borrow())) || (!(is_signed_59.with(|rc| *rc.borrow()))));
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_58.with(|rc| *rc.borrow())) || (!(is_signed_59.with(|rc| *rc.borrow()))));
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_58.with(|rc| *rc.borrow())) || (!(is_signed_59.with(|rc| *rc.borrow()))));
}
pub fn test_60(mut expected: f64) {
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_61.with(|rc| *rc.borrow())) || (!(is_signed_62.with(|rc| *rc.borrow()))));
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_61.with(|rc| *rc.borrow())) || (!(is_signed_62.with(|rc| *rc.borrow()))));
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_61.with(|rc| *rc.borrow())) || (!(is_signed_62.with(|rc| *rc.borrow()))));
    assert!((f64::MIN_POSITIVE == expected));
    assert!((is_bounded_61.with(|rc| *rc.borrow())) || (!(is_signed_62.with(|rc| *rc.borrow()))));
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
    ({ test_0(false) });
    ({ test_3(((-127_i32 - 1) as i8)) });
    ({ test_6(((-127_i32 - 1) as i8)) });
    ({ test_9(0_u8) });
    ({ test_12((-2147483647_i32 - 1)) });
    ({ test_15(0_u8) });
    ({ test_18(0_u16) });
    ({ test_21(0_u32) });
    ({ test_24(((-32767_i32 - 1) as i16)) });
    ({ test_27(0_u16) });
    ({ test_30((-2147483647_i32 - 1)) });
    ({ test_33(0_u32) });
    ({ test_36((-9223372036854775807_i64 - 1_i64)) });
    ({ test_39(0_u64) });
    ({ test_42((-9223372036854775807_i64 - 1_i64)) });
    ({ test_45(0_u64) });
    ({ test_48((-((((-1_i32 as u128) as u128).wrapping_div(2_u128)) as i128) - 1_i128)) });
    ({ test_51(0_u128) });
    ({ test_54(1.1754944e-38_f32) });
    ({ test_57(2.2250738585072014e-308_f64) });
    ({ test_60(0_f64) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
