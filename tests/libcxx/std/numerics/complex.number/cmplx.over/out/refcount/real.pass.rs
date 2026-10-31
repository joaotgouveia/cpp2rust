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
pub fn test_6(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (0 as f64)));
    let mut val: i32 = 0;
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_7(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (1 as f64)));
    let mut val: i32 = 1;
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_8(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (10 as f64)));
    let mut val: i32 = 10;
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_9(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (0 as f64)));
    let mut val: u32 = (0 as u32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_10(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (1 as f64)));
    let mut val: u32 = (1 as u32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_11(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (10 as f64)));
    let mut val: u32 = (10 as u32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_12(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (0 as f64)));
    let mut val: i64 = (0 as i64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_13(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (1 as f64)));
    let mut val: i64 = (1 as i64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_14(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!((unimplemented!() == (10 as f64)));
    let mut val: i64 = (10 as i64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_15(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (0 as f64)));
    let mut val: f32 = (0 as f32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_16(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (1 as f64)));
    let mut val: f32 = (1 as f32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_17(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (10 as f64)));
    let mut val: f32 = (10 as f32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_18(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (0 as f64)));
    let mut val: f64 = (0 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_19(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (1 as f64)));
    let mut val: f64 = (1 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_20(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (10 as f64)));
    let mut val: f64 = (10 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_21(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (0 as f64)));
    let mut val: f64 = (0 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_22(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (1 as f64)));
    let mut val: f64 = (1 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_23(_a0: Option<AnyPtr>) {
    let mut _a0: AnyPtr = _a0.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!((unimplemented!() == (10 as f64)));
    let mut val: f64 = (10 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_24() {
    ({ test_15(None) });
    ({ test_16(None) });
    ({ test_17(None) });
}
pub fn test_25() {
    ({ test_18(None) });
    ({ test_19(None) });
    ({ test_20(None) });
}
pub fn test_26() {
    ({ test_21(None) });
    ({ test_22(None) });
    ({ test_23(None) });
}
pub fn test_27() {
    ({ test_6(None) });
    ({ test_7(None) });
    ({ test_8(None) });
}
pub fn test_28() {
    ({ test_9(None) });
    ({ test_10(None) });
    ({ test_11(None) });
}
pub fn test_29() {
    ({ test_12(None) });
    ({ test_13(None) });
    ({ test_14(None) });
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
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
