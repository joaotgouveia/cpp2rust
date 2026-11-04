extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut expected: bool) {
    assert!(((({ std_numeric_limits_bool_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_bool_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_bool_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_bool_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_1(mut expected: i8) {
    assert!(((({ std_numeric_limits_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_2(mut expected: i8) {
    assert!(((({ std_numeric_limits_signed_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_signed_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_signed_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_signed_char_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_3(mut expected: u8) {
    assert!(((({ std_numeric_limits_unsigned_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_unsigned_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_unsigned_char_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_unsigned_char_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_4(mut expected: i32) {
    assert!((({ std_numeric_limits_wchar_t_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_wchar_t_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_wchar_t_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_wchar_t_::denorm_min() }) == expected));
}
pub fn test_5(mut expected: u8) {
    assert!(((({ std_numeric_limits_char8_t_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char8_t_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char8_t_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char8_t_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_6(mut expected: u16) {
    assert!(((({ std_numeric_limits_char16_t_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char16_t_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char16_t_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_char16_t_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_7(mut expected: u32) {
    assert!((({ std_numeric_limits_char32_t_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_char32_t_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_char32_t_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_char32_t_::denorm_min() }) == expected));
}
pub fn test_8(mut expected: i16) {
    assert!(((({ std_numeric_limits_short_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_short_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_short_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_short_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_9(mut expected: u16) {
    assert!(((({ std_numeric_limits_unsigned_short_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_unsigned_short_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_unsigned_short_::denorm_min() }) as i32) == (expected as i32)));
    assert!(((({ std_numeric_limits_unsigned_short_::denorm_min() }) as i32) == (expected as i32)));
}
pub fn test_10(mut expected: i32) {
    assert!((({ std_numeric_limits_int_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_int_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_int_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_int_::denorm_min() }) == expected));
}
pub fn test_11(mut expected: u32) {
    assert!((({ std_numeric_limits_unsigned_int_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_int_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_int_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_int_::denorm_min() }) == expected));
}
pub fn test_12(mut expected: i64) {
    assert!((({ std_numeric_limits_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_::denorm_min() }) == expected));
}
pub fn test_13(mut expected: u64) {
    assert!((({ std_numeric_limits_unsigned_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_long_::denorm_min() }) == expected));
}
pub fn test_14(mut expected: i64) {
    assert!((({ std_numeric_limits_long_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_long_::denorm_min() }) == expected));
}
pub fn test_15(mut expected: u64) {
    assert!((({ std_numeric_limits_unsigned_long_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_long_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_long_long_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned_long_long_::denorm_min() }) == expected));
}
pub fn test_16(mut expected: i128) {
    assert!((({ std_numeric_limits___int128_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits___int128_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits___int128_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits___int128_::denorm_min() }) == expected));
}
pub fn test_17(mut expected: u128) {
    assert!((({ std_numeric_limits_unsigned___int128_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned___int128_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned___int128_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_unsigned___int128_::denorm_min() }) == expected));
}
pub fn test_18(mut expected: f32) {
    assert!((({ std_numeric_limits_float_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_float_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_float_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_float_::denorm_min() }) == expected));
}
pub fn test_19(mut expected: f64) {
    assert!((({ std_numeric_limits_double_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_double_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_double_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_double_::denorm_min() }) == expected));
}
pub fn test_20(mut expected: f64) {
    assert!((({ std_numeric_limits_long_double_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_double_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_double_::denorm_min() }) == expected));
    assert!((({ std_numeric_limits_long_double_::denorm_min() }) == expected));
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
    ({ test_1(0_i8) });
    ({ test_2(0_i8) });
    ({ test_3(0_u8) });
    ({ test_4(0_i32) });
    ({ test_5(0_u8) });
    ({ test_6(0_u16) });
    ({ test_7(0_u32) });
    ({ test_8(0_i16) });
    ({ test_9(0_u16) });
    ({ test_10(0) });
    ({ test_11(0_u32) });
    ({ test_12(0_i64) });
    ({ test_13(0_u64) });
    ({ test_14(0_i64) });
    ({ test_15(0_u64) });
    ({ test_16(0_i128) });
    ({ test_17(0_u128) });
    ({ test_18(1e-45_f32) });
    ({ test_19(5e-324_f64) });
    ({ test_20(0_f64) });
    ({ test_18(1e-45_f32) });
    ({ test_19(5e-324_f64) });
    ({ test_20(0_f64) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
