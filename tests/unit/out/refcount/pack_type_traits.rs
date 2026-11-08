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
fn main_0() -> i32 {
    assert!(<() as libcc2rs::Conjunction>::VALUE);
    assert!(<(libcc2rs::IntegralConstant<bool, 1>,) as libcc2rs::Conjunction>::VALUE);
    assert!(!(<(libcc2rs::IntegralConstant<bool, 0>,) as libcc2rs::Conjunction>::VALUE));
    assert!(
        (<(
            libcc2rs::IntegralConstant<bool, 1>,
            libcc2rs::IntegralConstant<bool, 1>,
        ) as libcc2rs::Conjunction>::VALUE)
    );
    assert!(
        (!(<(
            libcc2rs::IntegralConstant<bool, 1>,
            libcc2rs::IntegralConstant<bool, 0>,
        ) as libcc2rs::Conjunction>::VALUE))
    );
    assert!(!(<() as libcc2rs::Disjunction>::VALUE));
    assert!(<(libcc2rs::IntegralConstant<bool, 1>,) as libcc2rs::Disjunction>::VALUE);
    assert!(!(<(libcc2rs::IntegralConstant<bool, 0>,) as libcc2rs::Disjunction>::VALUE));
    assert!(
        (<(
            libcc2rs::IntegralConstant<bool, 0>,
            libcc2rs::IntegralConstant<bool, 1>,
        ) as libcc2rs::Disjunction>::VALUE)
    );
    assert!(
        (!(<(
            libcc2rs::IntegralConstant<bool, 0>,
            libcc2rs::IntegralConstant<bool, 0>,
        ) as libcc2rs::Disjunction>::VALUE))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
