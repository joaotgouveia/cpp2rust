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
    if unimplemented!() {
        return (anon_1_inf as ());
    }
    if unimplemented!() {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub fn is_about_3(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_4(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_6(mut x: (), _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    unimplemented!();
}
pub fn test_7(mut x: (), _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    unimplemented!();
}
pub fn test_8(mut x: (), _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    unimplemented!();
}
pub fn test_9(mut x: (), _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    unimplemented!();
}
pub fn test_10(mut x: (), _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    unimplemented!();
}
pub fn test_11(mut x: (), _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    unimplemented!();
}
pub fn test_12() {
    ({ test_9(0_(), None) });
    ({ test_9(1_(), None) });
    ({ test_9(10_(), None) });
}
pub fn test_13() {
    ({ test_10(0_(), None) });
    ({ test_10(1_(), None) });
    ({ test_10(10_(), None) });
}
pub fn test_14() {
    ({ test_11(0_(), None) });
    ({ test_11(1_(), None) });
    ({ test_11(10_(), None) });
}
pub fn test_15() {
    ({ test_6(0_(), None) });
    ({ test_6(1_(), None) });
    ({ test_6(10_(), None) });
}
pub fn test_16() {
    ({ test_7(0_(), None) });
    ({ test_7(1_(), None) });
    ({ test_7(10_(), None) });
}
pub fn test_17() {
    ({ test_8(0_(), None) });
    ({ test_8(1_(), None) });
    ({ test_8(10_(), None) });
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
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
