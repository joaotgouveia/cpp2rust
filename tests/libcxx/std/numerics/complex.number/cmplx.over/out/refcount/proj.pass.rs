extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static testcases_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
);
pub type anon_1 = u32;
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: f64) -> i32 {
    if (x == 0_f64) {
        return (anon_1_zero as i32);
    }
    if unimplemented!() {
        return (anon_1_inf as i32);
    }
    if unimplemented!() {
        return (anon_1_NaN as i32);
    }
    return (anon_1_non_zero as i32);
}
pub fn is_about_3(mut x: f32, mut y: f32) {
    assert!(((unimplemented!() as f64) < 1e-06_f64));
}
pub fn is_about_4(mut x: f64, mut y: f64) {
    assert!((unimplemented!() < 1e-14_f64));
}
pub fn is_about_5(mut x: f64, mut y: f64) {
    assert!((unimplemented!() < 1e-14_f64));
}
pub fn test_6(mut x: i32, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<double> >::value)"
    );;
    assert!(unimplemented!());
}
pub fn test_7(mut x: u32, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<double> >::value)"
    );;
    assert!(unimplemented!());
}
pub fn test_8(mut x: i64, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<double> >::value)"
    );;
    assert!(unimplemented!());
}
pub fn test_9(mut x: f32, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<T> >::value)"
    );;
    assert!(unimplemented!());
}
pub fn test_10(mut x: f64, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<T> >::value)"
    );;
    assert!(unimplemented!());
}
pub fn test_11(mut x: f64, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<T> >::value)"
    );;
    assert!(unimplemented!());
}
pub fn test_12() {
    ({ test_9(0_f32, None) });
    ({ test_9(1_f32, None) });
    ({ test_9(10_f32, None) });
}
pub fn test_13() {
    ({ test_10(0_f64, None) });
    ({ test_10(1_f64, None) });
    ({ test_10(10_f64, None) });
}
pub fn test_14() {
    ({ test_11(0_f64, None) });
    ({ test_11(1_f64, None) });
    ({ test_11(10_f64, None) });
}
pub fn test_15() {
    ({ test_6(0, None) });
    ({ test_6(1, None) });
    ({ test_6(10, None) });
}
pub fn test_16() {
    ({ test_7(0_u32, None) });
    ({ test_7(1_u32, None) });
    ({ test_7(10_u32, None) });
}
pub fn test_17() {
    ({ test_8(0_i64, None) });
    ({ test_8(1_i64, None) });
    ({ test_8(10_i64, None) });
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
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
