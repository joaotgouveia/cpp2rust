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
pub type anon_1 = ();
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: ()) -> () {
    if (x == 0_()) {
        return (anon_1_zero as ());
    }
    if ({ isinf_3(x) }) {
        return (anon_1_inf as ());
    }
    if ({ isnan_4(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_8(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 0_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_9(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 1_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_10(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 10_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_11(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 0_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_12(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 1_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_13(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 10_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_14(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 0_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_15(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 1_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_16(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    unimplemented!();
    let mut val: () = 10_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_17(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 0_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_18(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 1_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_19(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 10_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_20(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 0_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_21(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 1_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_22(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 10_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_23(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 0_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_24(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 1_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_25(_a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    unimplemented!();
    let mut val: () = 10_();
    const _: () = assert!(true, "std::real(val) == val");;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "t.real() == x");;
}
pub fn test_26() {
    ({ test_17(None) });
    ({ test_18(None) });
    ({ test_19(None) });
}
pub fn test_27() {
    ({ test_20(None) });
    ({ test_21(None) });
    ({ test_22(None) });
}
pub fn test_28() {
    ({ test_23(None) });
    ({ test_24(None) });
    ({ test_25(None) });
}
pub fn test_29() {
    ({ test_8(None) });
    ({ test_9(None) });
    ({ test_10(None) });
}
pub fn test_30() {
    ({ test_11(None) });
    ({ test_12(None) });
    ({ test_13(None) });
}
pub fn test_31() {
    ({ test_14(None) });
    ({ test_15(None) });
    ({ test_16(None) });
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
    ({ test_26() });
    ({ test_27() });
    ({ test_28() });
    ({ test_29() });
    ({ test_30() });
    ({ test_31() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
