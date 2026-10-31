extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut expected: bool) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_1(mut expected: i8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_2(mut expected: i8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_3(mut expected: u8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_4(mut expected: i32) {
    assert!(((unimplemented!() as i32) == expected));
    assert!(((unimplemented!() as i32) == expected));
    assert!(((unimplemented!() as i32) == expected));
    assert!(((unimplemented!() as i32) == expected));
}
pub fn test_5(mut expected: u8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_6(mut expected: u16) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_7(mut expected: u32) {
    assert!(((unimplemented!() as u32) == expected));
    assert!(((unimplemented!() as u32) == expected));
    assert!(((unimplemented!() as u32) == expected));
    assert!(((unimplemented!() as u32) == expected));
}
pub fn test_8(mut expected: i16) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_9(mut expected: u16) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub fn test_10(mut expected: i32) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_11(mut expected: u32) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_12(mut expected: i64) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_13(mut expected: u64) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_14(mut expected: i64) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_15(mut expected: u64) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_16(mut expected: i128) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_17(mut expected: u128) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_18(mut expected: f32) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_19(mut expected: f64) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
}
pub fn test_20(mut expected: f64) {
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
    assert!((unimplemented!() == expected));
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
    ({ test_18((1_f32 / zero_21.with(|rc| *rc.borrow()))) });
    ({ test_19((1_f64 / (zero_21.with(|rc| *rc.borrow()) as f64))) });
    ({ test_20((1_f64 / (zero_21.with(|rc| *rc.borrow()) as f64))) });
    return 0;
}
thread_local!(
    pub static zero_21: Value<f32> = Rc::new(RefCell::new(0_f32));
);
pub fn __cpp2rust_init_globals() {
    let _ = zero_21.with(|_| ());
}
