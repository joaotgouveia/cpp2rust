extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a0: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___) {
    let a0: Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___> =
        Rc::new(RefCell::new(a0));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vec_: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ 4_u64 }, None),
    ));
    let it: Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___> =
        Rc::new(RefCell::new(({ (*vec_.borrow()).begin_28() })));
    ({ foo_0((*it.borrow()).clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
