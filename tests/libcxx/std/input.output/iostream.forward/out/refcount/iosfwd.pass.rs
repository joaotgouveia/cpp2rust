extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_1() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_2() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_3() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_4() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_5() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_6() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_7() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_8() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_9() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_10() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_11() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_12() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_13() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_14() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_15() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_16() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_17() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_18() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_19() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_20() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_21() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_22() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_23() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_24() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_25() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_26() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_27() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_28() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_29() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_30() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_31() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
}
pub fn test_32() {
    let mut p: Ptr<()> = Ptr::<()>::null();
    (&(p));
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    ({ test_9() });
    ({ test_10() });
    ({ test_11() });
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    ({ test_18() });
    ({ test_19() });
    ({ test_20() });
    ({ test_21() });
    ({ test_22() });
    ({ test_23() });
    ({ test_24() });
    ({ test_25() });
    ({ test_26() });
    ({ test_27() });
    ({ test_28() });
    ({ test_29() });
    ({ test_30() });
    ({ test_31() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_6() });
    ({ test_8() });
    ({ test_10() });
    ({ test_12() });
    ({ test_14() });
    ({ test_16() });
    ({ test_18() });
    ({ test_20() });
    ({ test_22() });
    ({ test_24() });
    ({ test_26() });
    ({ test_5() });
    ({ test_7() });
    ({ test_9() });
    ({ test_11() });
    ({ test_13() });
    ({ test_15() });
    ({ test_17() });
    ({ test_19() });
    ({ test_21() });
    ({ test_23() });
    ({ test_25() });
    ({ test_27() });
    ({ test_32() });
    ({ test_32() });
    ({ test_32() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
