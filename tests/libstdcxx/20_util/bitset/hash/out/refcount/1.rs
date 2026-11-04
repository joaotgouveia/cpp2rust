extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let b0: Value<std_bitset_0_> = Rc::new(RefCell::new(<std_bitset_0_>::default()));
    let h0: Value<std_hash_std_bitset_0__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_0__>::default()));
    ({ (*h0.borrow_mut())(b0.as_pointer()) });
    let b1: Value<std_bitset_10_> = Rc::new(RefCell::new(<std_bitset_10_>::default()));
    let h1: Value<std_hash_std_bitset_10__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_10__>::default()));
    ({ (*h1.borrow_mut())(b1.as_pointer()) });
    let b2: Value<std_bitset_100_> = Rc::new(RefCell::new(<std_bitset_100_>::default()));
    let h2: Value<std_hash_std_bitset_100__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_100__>::default()));
    ({ (*h2.borrow_mut())(b2.as_pointer()) });
    let b3: Value<std_bitset_1000_> = Rc::new(RefCell::new(<std_bitset_1000_>::default()));
    let h3: Value<std_hash_std_bitset_1000__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_1000__>::default()));
    ({ (*h3.borrow_mut())(b3.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
