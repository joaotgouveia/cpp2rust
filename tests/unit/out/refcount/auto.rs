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
    let mut x1: () = 1_();
    let mut x2: () = 2_();
    let mut x3: () = 4_();
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(1_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(2_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    let mut sum: () = 0_();
    'loop_: for elem in IterRange::new(({ (*v.borrow()).begin_26() }), ({ (*v.borrow()).end_28() }))
    {
        let mut elem: () = (elem.read());
        sum += elem;
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
