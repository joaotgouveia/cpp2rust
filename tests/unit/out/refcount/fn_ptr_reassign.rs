extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_0(mut a: (), mut b: ()) -> () {
    return (a + b);
}
pub fn sub_1(mut a: (), mut b: ()) -> () {
    return (a - b);
}
pub fn mul_2(mut a: (), mut b: ()) -> () {
    return (a * b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut fn_: FnPtr<fn((), ()) -> ()> = FnPtr::<fn((), ()) -> ()>::new(add_0);
    unimplemented!();
    fn_ = FnPtr::<fn((), ()) -> ()>::new(sub_1);
    unimplemented!();
    fn_ = FnPtr::<fn((), ()) -> ()>::new(mul_2);
    unimplemented!();
    fn_ = FnPtr::<fn((), ()) -> ()>::null();
    unimplemented!();
    fn_ = FnPtr::<fn((), ()) -> ()>::new(add_0);
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
