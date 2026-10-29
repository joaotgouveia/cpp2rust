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
    let v : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____  >::default() )) ;
    ({
        let ___x: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_vector_int__std_allocator_int__::new_1({ 10_() }, None),
        ));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
