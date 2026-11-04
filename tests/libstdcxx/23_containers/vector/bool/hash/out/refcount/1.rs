extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let b0: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
    let h0: Value<std_hash_std_vector_bool__std_allocator_bool___> = Rc::new(RefCell::new(
        <std_hash_std_vector_bool__std_allocator_bool___>::default(),
    ));
    ({ (*h0.borrow_mut())(b0.as_pointer()) });
    let b1: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 10_usize]));
    let h1: Value<std_hash_std_vector_bool__std_allocator_bool___> = Rc::new(RefCell::new(
        <std_hash_std_vector_bool__std_allocator_bool___>::default(),
    ));
    ({ (*h1.borrow_mut())(b1.as_pointer()) });
    let b2: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 100_usize]));
    let h2: Value<std_hash_std_vector_bool__std_allocator_bool___> = Rc::new(RefCell::new(
        <std_hash_std_vector_bool__std_allocator_bool___>::default(),
    ));
    ({ (*h2.borrow_mut())(b2.as_pointer()) });
    let b3: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 1000_usize]));
    let h3: Value<std_hash_std_vector_bool__std_allocator_bool___> = Rc::new(RefCell::new(
        <std_hash_std_vector_bool__std_allocator_bool___>::default(),
    ));
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
