extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut expected: bool) {
    assert!(((true as i32) == (expected as i32)));
    assert!(is_bounded_1.with(|rc| *rc.borrow()));
    assert!(((true as i32) == (expected as i32)));
    assert!(is_bounded_1.with(|rc| *rc.borrow()));
    assert!(((true as i32) == (expected as i32)));
    assert!(is_bounded_1.with(|rc| *rc.borrow()));
    assert!(((true as i32) == (expected as i32)));
    assert!(is_bounded_1.with(|rc| *rc.borrow()));
}
pub fn test_2(mut expected: i8) {
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(is_bounded_3.with(|rc| *rc.borrow()));
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(is_bounded_3.with(|rc| *rc.borrow()));
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(is_bounded_3.with(|rc| *rc.borrow()));
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(is_bounded_3.with(|rc| *rc.borrow()));
}
pub fn test_4(mut expected: i8) {
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_5.with(|rc| *rc.borrow()));
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_5.with(|rc| *rc.borrow()));
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_5.with(|rc| *rc.borrow()));
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_5.with(|rc| *rc.borrow()));
}
pub fn test_6(mut expected: u8) {
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_7.with(|rc| *rc.borrow()));
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_7.with(|rc| *rc.borrow()));
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_7.with(|rc| *rc.borrow()));
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_7.with(|rc| *rc.borrow()));
}
pub fn test_8(mut expected: i32) {
    assert!((i32::MAX == expected));
    assert!(is_bounded_9.with(|rc| *rc.borrow()));
    assert!((i32::MAX == expected));
    assert!(is_bounded_9.with(|rc| *rc.borrow()));
    assert!((i32::MAX == expected));
    assert!(is_bounded_9.with(|rc| *rc.borrow()));
    assert!((i32::MAX == expected));
    assert!(is_bounded_9.with(|rc| *rc.borrow()));
}
pub fn test_10(mut expected: u8) {
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_11.with(|rc| *rc.borrow()));
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_11.with(|rc| *rc.borrow()));
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_11.with(|rc| *rc.borrow()));
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(is_bounded_11.with(|rc| *rc.borrow()));
}
pub fn test_12(mut expected: u16) {
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_13.with(|rc| *rc.borrow()));
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_13.with(|rc| *rc.borrow()));
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_13.with(|rc| *rc.borrow()));
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_13.with(|rc| *rc.borrow()));
}
pub fn test_14(mut expected: u32) {
    assert!((u32::MAX == expected));
    assert!(is_bounded_15.with(|rc| *rc.borrow()));
    assert!((u32::MAX == expected));
    assert!(is_bounded_15.with(|rc| *rc.borrow()));
    assert!((u32::MAX == expected));
    assert!(is_bounded_15.with(|rc| *rc.borrow()));
    assert!((u32::MAX == expected));
    assert!(is_bounded_15.with(|rc| *rc.borrow()));
}
pub fn test_16(mut expected: i16) {
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_17.with(|rc| *rc.borrow()));
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_17.with(|rc| *rc.borrow()));
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_17.with(|rc| *rc.borrow()));
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_17.with(|rc| *rc.borrow()));
}
pub fn test_18(mut expected: u16) {
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_19.with(|rc| *rc.borrow()));
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_19.with(|rc| *rc.borrow()));
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_19.with(|rc| *rc.borrow()));
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(is_bounded_19.with(|rc| *rc.borrow()));
}
pub fn test_20(mut expected: i32) {
    assert!((i32::MAX == expected));
    assert!(is_bounded_21.with(|rc| *rc.borrow()));
    assert!((i32::MAX == expected));
    assert!(is_bounded_21.with(|rc| *rc.borrow()));
    assert!((i32::MAX == expected));
    assert!(is_bounded_21.with(|rc| *rc.borrow()));
    assert!((i32::MAX == expected));
    assert!(is_bounded_21.with(|rc| *rc.borrow()));
}
pub fn test_22(mut expected: u32) {
    assert!((u32::MAX == expected));
    assert!(is_bounded_23.with(|rc| *rc.borrow()));
    assert!((u32::MAX == expected));
    assert!(is_bounded_23.with(|rc| *rc.borrow()));
    assert!((u32::MAX == expected));
    assert!(is_bounded_23.with(|rc| *rc.borrow()));
    assert!((u32::MAX == expected));
    assert!(is_bounded_23.with(|rc| *rc.borrow()));
}
pub fn test_24(mut expected: i64) {
    assert!((i64::MAX == expected));
    assert!(is_bounded_25.with(|rc| *rc.borrow()));
    assert!((i64::MAX == expected));
    assert!(is_bounded_25.with(|rc| *rc.borrow()));
    assert!((i64::MAX == expected));
    assert!(is_bounded_25.with(|rc| *rc.borrow()));
    assert!((i64::MAX == expected));
    assert!(is_bounded_25.with(|rc| *rc.borrow()));
}
pub fn test_26(mut expected: u64) {
    assert!((u64::MAX == expected));
    assert!(is_bounded_27.with(|rc| *rc.borrow()));
    assert!((u64::MAX == expected));
    assert!(is_bounded_27.with(|rc| *rc.borrow()));
    assert!((u64::MAX == expected));
    assert!(is_bounded_27.with(|rc| *rc.borrow()));
    assert!((u64::MAX == expected));
    assert!(is_bounded_27.with(|rc| *rc.borrow()));
}
pub fn test_28(mut expected: i64) {
    assert!((i64::MAX == expected));
    assert!(is_bounded_29.with(|rc| *rc.borrow()));
    assert!((i64::MAX == expected));
    assert!(is_bounded_29.with(|rc| *rc.borrow()));
    assert!((i64::MAX == expected));
    assert!(is_bounded_29.with(|rc| *rc.borrow()));
    assert!((i64::MAX == expected));
    assert!(is_bounded_29.with(|rc| *rc.borrow()));
}
pub fn test_30(mut expected: u64) {
    assert!((<u64>::MAX == expected));
    assert!(is_bounded_31.with(|rc| *rc.borrow()));
    assert!((<u64>::MAX == expected));
    assert!(is_bounded_31.with(|rc| *rc.borrow()));
    assert!((<u64>::MAX == expected));
    assert!(is_bounded_31.with(|rc| *rc.borrow()));
    assert!((<u64>::MAX == expected));
    assert!(is_bounded_31.with(|rc| *rc.borrow()));
}
pub fn test_32(mut expected: i128) {
    assert!((i128::MAX == expected));
    assert!(is_bounded_33.with(|rc| *rc.borrow()));
    assert!((i128::MAX == expected));
    assert!(is_bounded_33.with(|rc| *rc.borrow()));
    assert!((i128::MAX == expected));
    assert!(is_bounded_33.with(|rc| *rc.borrow()));
    assert!((i128::MAX == expected));
    assert!(is_bounded_33.with(|rc| *rc.borrow()));
}
pub fn test_34(mut expected: u128) {
    assert!((<u128>::MAX == expected));
    assert!(is_bounded_35.with(|rc| *rc.borrow()));
    assert!((<u128>::MAX == expected));
    assert!(is_bounded_35.with(|rc| *rc.borrow()));
    assert!((<u128>::MAX == expected));
    assert!(is_bounded_35.with(|rc| *rc.borrow()));
    assert!((<u128>::MAX == expected));
    assert!(is_bounded_35.with(|rc| *rc.borrow()));
}
pub fn test_36(mut expected: f32) {
    assert!((f32::MAX == expected));
    assert!(is_bounded_37.with(|rc| *rc.borrow()));
    assert!((f32::MAX == expected));
    assert!(is_bounded_37.with(|rc| *rc.borrow()));
    assert!((f32::MAX == expected));
    assert!(is_bounded_37.with(|rc| *rc.borrow()));
    assert!((f32::MAX == expected));
    assert!(is_bounded_37.with(|rc| *rc.borrow()));
}
pub fn test_38(mut expected: f64) {
    assert!((f64::MAX == expected));
    assert!(is_bounded_39.with(|rc| *rc.borrow()));
    assert!((f64::MAX == expected));
    assert!(is_bounded_39.with(|rc| *rc.borrow()));
    assert!((f64::MAX == expected));
    assert!(is_bounded_39.with(|rc| *rc.borrow()));
    assert!((f64::MAX == expected));
    assert!(is_bounded_39.with(|rc| *rc.borrow()));
}
pub fn test_40(mut expected: f64) {
    assert!((f64::MAX == expected));
    assert!(is_bounded_41.with(|rc| *rc.borrow()));
    assert!((f64::MAX == expected));
    assert!(is_bounded_41.with(|rc| *rc.borrow()));
    assert!((f64::MAX == expected));
    assert!(is_bounded_41.with(|rc| *rc.borrow()));
    assert!((f64::MAX == expected));
    assert!(is_bounded_41.with(|rc| *rc.borrow()));
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
    ({ test_0(true) });
    ({ test_2(127_i8) });
    ({ test_4(127_i8) });
    ({ test_6((((127 * 2) + 1) as u8)) });
    ({ test_8(2147483647_i32) });
    ({ test_10((((127 * 2) + 1) as u8)) });
    ({ test_12((((32767 * 2) + 1) as u16)) });
    ({
        test_14(
            (((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32).wrapping_add((1_u32 as u32))),
        )
    });
    ({ test_16(32767_i16) });
    ({ test_18((((32767 * 2) + 1) as u16)) });
    ({ test_20(2147483647) });
    ({
        test_22(
            (((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32).wrapping_add((1_u32 as u32))),
        )
    });
    ({ test_24(9223372036854775807_i64) });
    ({
        test_26(
            (((9223372036854775807_u64).wrapping_mul((2_u64 as u64)) as u64)
                .wrapping_add((1_u64 as u64))),
        )
    });
    ({ test_28(9223372036854775807_i64) });
    ({
        test_30(
            (((9223372036854775807_u64).wrapping_mul((2_u64 as u64)) as u64)
                .wrapping_add((1_u64 as u64))),
        )
    });
    ({ test_32(((((-1_i32 as u128) as u128).wrapping_div(2_u128)) as i128)) });
    ({ test_34((-1_i32 as u128)) });
    ({ test_36(3.4028235e+38_f32) });
    ({ test_38(1.7976931348623157e+308_f64) });
    ({ test_40(f64::INFINITY) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
