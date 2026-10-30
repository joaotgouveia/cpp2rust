extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn testConstexprSpan_0(sp: (), mut idx: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    return ({ (r1.read()) } == { (r2.read()) });
}
pub fn testConstexprSpan_1(sp: (), mut idx: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    return ({ (r1.read()) } == { (r2.read()) });
}
pub fn testConstexprSpan_2(sp: (), mut idx: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    return ({ (r1.read()) } == { (r2.read()) });
}
pub fn testConstexprSpan_3(sp: (), mut idx: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    return ({ (r1.read()) } == { (r2.read()) });
}
pub fn testConstexprSpan_4(sp: (), mut idx: ()) -> () {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    return ({ (r1.read()) } == { (r2.read()) });
}
pub fn testRuntimeSpan_5(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
pub fn testRuntimeSpan_6(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
pub fn testRuntimeSpan_7(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
pub fn testRuntimeSpan_8(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
pub fn testRuntimeSpan_9(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
pub fn testRuntimeSpan_10(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
pub fn testRuntimeSpan_11(sp: (), mut idx: ()) {
    let sp: Value<()> = Rc::new(RefCell::new(sp));
    const _: () = assert!(true, "");;
    let r1: () = unimplemented!();
    let r2: () = (unimplemented!().offset((idx) as isize));
    unimplemented!();
}
thread_local!(
    pub static iArr1_12: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
    ])));
);
thread_local!(
    pub static iArr2_13: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
        16_(),
        17_(),
        18_(),
        19_(),
    ])));
);

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
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 1), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 2)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 2)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 3)");;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 1>(iArr1, 1), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 2>(iArr1, 2), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 2>(iArr1, 2), 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), 2)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 2)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 3)"
    );;
    ({ testRuntimeSpan_5(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 1_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 1_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 2_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 1_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 2_()) });
    ({ testRuntimeSpan_5(unimplemented!(), 3_()) });
    ({ testRuntimeSpan_6(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_7(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_7(unimplemented!(), 1_()) });
    ({ testRuntimeSpan_8(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_8(unimplemented!(), 1_()) });
    ({ testRuntimeSpan_8(unimplemented!(), 2_()) });
    ({ testRuntimeSpan_9(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_9(unimplemented!(), 1_()) });
    ({ testRuntimeSpan_9(unimplemented!(), 2_()) });
    ({ testRuntimeSpan_9(unimplemented!(), 3_()) });
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ testRuntimeSpan_10(unimplemented!(), 0_()) });
    ({ testRuntimeSpan_11(unimplemented!(), 0_()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = iArr1_12.with(|_| ());
    let _ = iArr2_13.with(|_| ());
}
