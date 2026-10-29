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
    let v: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        <std_vector_unsigned_int__std_allocator_unsigned_int__>::default(),
    ));
    let mut bytes: Ptr<()> = ({ (*v.borrow()).data_55() }).reinterpret_cast::<()>();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
