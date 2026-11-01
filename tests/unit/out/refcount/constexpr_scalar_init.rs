extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: i32) -> i32 {
    return (x * x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut ce: bool = true;
    assert!(ce);
    let mut rt: bool = false;
    assert!(!(rt));
    let mut n: i32 = -1_i32;
    assert!((n == -1_i32));
    let mut d: f64 = 5_f64;
    assert!((d == 5_f64));
    let mut big: u64 = 1099511627776_u64;
    assert!((big == 1099511627776_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
