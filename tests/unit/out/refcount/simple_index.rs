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
    let v: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new(
        std_vector_bool__std_allocator_bool__::new_1({ unimplemented!() }, None),
    ));
    unimplemented!();
    let r: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[0_()]).clone()));
    let copy: Value<std__Bit_reference> = Rc::new(RefCell::new((*r.borrow()).clone()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
