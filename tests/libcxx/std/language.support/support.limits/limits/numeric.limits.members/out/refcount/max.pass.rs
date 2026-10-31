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
    assert!(unimplemented!());
    assert!(((true as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((true as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((true as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_1(mut expected: i8) {
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((libc::c_char::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_2(mut expected: i8) {
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((i8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_3(mut expected: u8) {
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_4(mut expected: i32) {
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_5(mut expected: u8) {
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u8::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_6(mut expected: u16) {
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_7(mut expected: u32) {
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_8(mut expected: i16) {
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((i16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_9(mut expected: u16) {
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
    assert!(((u16::MAX as i32) == (expected as i32)));
    assert!(unimplemented!());
}
pub fn test_10(mut expected: i32) {
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
    assert!((i32::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_11(mut expected: u32) {
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
    assert!((u32::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_12(mut expected: i64) {
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_13(mut expected: u64) {
    assert!((u64::MAX == expected));
    assert!(unimplemented!());
    assert!((u64::MAX == expected));
    assert!(unimplemented!());
    assert!((u64::MAX == expected));
    assert!(unimplemented!());
    assert!((u64::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_14(mut expected: i64) {
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
    assert!((i64::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_15(mut expected: u64) {
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
}
pub fn test_16(mut expected: i128) {
    assert!((i128::MAX == expected));
    assert!(unimplemented!());
    assert!((i128::MAX == expected));
    assert!(unimplemented!());
    assert!((i128::MAX == expected));
    assert!(unimplemented!());
    assert!((i128::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_17(mut expected: u128) {
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
    assert!((unimplemented!() == expected));
    assert!(unimplemented!());
}
pub fn test_18(mut expected: f32) {
    assert!((f32::MAX == expected));
    assert!(unimplemented!());
    assert!((f32::MAX == expected));
    assert!(unimplemented!());
    assert!((f32::MAX == expected));
    assert!(unimplemented!());
    assert!((f32::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_19(mut expected: f64) {
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
}
pub fn test_20(mut expected: f64) {
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
    assert!((f64::MAX == expected));
    assert!(unimplemented!());
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
    ({ test_1(127_i8) });
    ({ test_2(127_i8) });
    ({ test_3((((127 * 2) + 1) as u8)) });
    ({ test_4(2147483647_i32) });
    ({ test_5((((127 * 2) + 1) as u8)) });
    ({ test_6((((32767 * 2) + 1) as u16)) });
    ({
        test_7(
            (((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32).wrapping_add((1_u32 as u32))),
        )
    });
    ({ test_8(32767_i16) });
    ({ test_9((((32767 * 2) + 1) as u16)) });
    ({ test_10(2147483647) });
    ({
        test_11(
            (((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32).wrapping_add((1_u32 as u32))),
        )
    });
    ({ test_12(9223372036854775807_i64) });
    ({
        test_13(
            (((9223372036854775807_u64).wrapping_mul((2_u64 as u64)) as u64)
                .wrapping_add((1_u64 as u64))),
        )
    });
    ({ test_14(9223372036854775807_i64) });
    ({
        test_15(
            (((9223372036854775807_u64).wrapping_mul((2_u64 as u64)) as u64)
                .wrapping_add((1_u64 as u64))),
        )
    });
    ({ test_16(((((-1_i32 as u128) as u128).wrapping_div(2_u128)) as i128)) });
    ({ test_17((-1_i32 as u128)) });
    ({ test_18(3.4028235e+38_f32) });
    ({ test_19(1.7976931348623157e+308_f64) });
    ({ test_20(f64::INFINITY) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
