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
    let arr2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([2_(), 2_()])));
    (*arr2.borrow_mut())[(unimplemented!()) as usize] = 3_();
    (*arr2.borrow_mut())[(1_()) as usize] = 4_();
    let mut arr2_ptr: Ptr<()> = (arr2.as_pointer() as Ptr<()>);
    elem!(arr2_ptr, unimplemented!()).write(5_());
    elem!(arr2_ptr, 1_()).write(6_());
    let arr2_ref1: Ptr<()> = (arr2.as_pointer() as Ptr<()>).offset(1_());
    arr2_ref1.write(7_());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
