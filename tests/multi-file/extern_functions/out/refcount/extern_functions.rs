extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub fn unrelated1_0() -> () {
    return 1_();
}
pub fn unrelated2_1() -> () {
    return 2_();
}
pub fn unrelated3_2() -> () {
    return 3_();
}
pub fn helper_3(mut x: ()) -> () {
    &({ unrelated1_0() });
    &({ unrelated2_1() });
    &({ unrelated3_2() });
    return (x + 1_());
}
pub fn __cpp2rust_init_globals() {}
