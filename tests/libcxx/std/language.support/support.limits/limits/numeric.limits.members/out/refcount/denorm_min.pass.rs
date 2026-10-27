extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_1(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_2(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_3(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_4(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_5(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_6(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_7(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_8(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_9(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_10(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_11(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_12(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_13(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_14(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_15(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_16(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_17(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_18(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_19(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_20(mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_0(false) });
    ({ test_1(0_()) });
    ({ test_2(0_()) });
    ({ test_3(0_()) });
    ({ test_4(0_()) });
    ({ test_5(0_()) });
    ({ test_6(0_()) });
    ({ test_7(0_()) });
    ({ test_8(0_()) });
    ({ test_9(0_()) });
    ({ test_10(0_()) });
    ({ test_11(0_()) });
    ({ test_12(0_()) });
    ({ test_13(0_()) });
    ({ test_14(0_()) });
    ({ test_15(0_()) });
    ({ test_16(0_()) });
    ({ test_17(0_()) });
    ({ test_18(1e-45_f32) });
    ({ test_19(5e-324_f64) });
    ({ test_20(0_f64) });
    ({ test_18(1e-45_f32) });
    ({ test_19(5e-324_f64) });
    ({ test_20(0_f64) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
