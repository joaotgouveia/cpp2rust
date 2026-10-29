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
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
    let p: Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___> =
        Rc::new(RefCell::new(({ (*v.borrow()).begin_26() })));
    let r: Ptr<()> = v.as_pointer().offset(1_());
    let __rhs = (r.read());
    (*p.borrow_mut()).write(__rhs);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
