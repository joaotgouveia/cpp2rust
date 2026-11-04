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
    let v: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(RefCell::new(
        <std_vector_unsigned_int__std_allocator_unsigned_int__>::default(),
    ));
    let mut bytes: Ptr<u16> = ({ (*v.borrow()).data_57() }).reinterpret_cast::<u16>();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
