extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(bytes: std_initializer_list_int_) -> () {
    let bytes: Value<std_initializer_list_int_> = Rc::new(RefCell::new(bytes));
    let mut buf: Ptr<std_vector_int__std_allocator_int__> = Ptr::alloc(
        std_vector_int__std_allocator_int__::new_1({ (*bytes.borrow()).clone() }, None),
    );
    let mut n: () = ({ (*bytes.borrow()).size() });
    buf.delete();
    return n;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
    unimplemented!();
    unimplemented!();
    let l: Value<std_initializer_list_int_> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
