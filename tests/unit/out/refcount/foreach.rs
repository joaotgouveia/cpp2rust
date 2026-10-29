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
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((*i.borrow()) < 10_()) {
        ({ (*v.borrow()).push_back_57(i.as_pointer()) });
        (*i.borrow_mut()).prefix_inc();
    }
    let mut sum: () = 0_();
    'loop_: for x in IterRange::new(({ (*v.borrow()).begin_26() }), ({ (*v.borrow()).end_28() })) {
        let mut x: () = (x.read());
        sum += x;
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
