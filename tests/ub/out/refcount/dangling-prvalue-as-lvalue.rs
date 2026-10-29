extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: Ptr<()>) -> Ptr<()> {
    return (a).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
    let b: Ptr<()> =
        ({ foo_0(Rc::new(RefCell::new(({ (*v.borrow()).begin_26() }))).as_pointer()) });
    ({ (*v.borrow()).clear() });
    return (b.read());
}
pub fn __cpp2rust_init_globals() {}
