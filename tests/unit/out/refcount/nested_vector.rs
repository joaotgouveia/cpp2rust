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
    let v1 : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____  >::default() )) ;
    let v2: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
    ({ (*v1.borrow()).push_back_57(v2.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
