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
    let p1: Value<()> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
        let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
        std_pair_double__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    unimplemented!();
    unimplemented!();
    let p2: Value<()> = Rc::new(RefCell::new({
        let __tmp_2: Value<()> = Rc::new(RefCell::new(3_()));
        let __tmp_3: Value<()> = Rc::new(RefCell::new(4_()));
        std_pair_double__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    }));
    unimplemented!();
    unimplemented!();
    let p3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    let mut n: () = 2_();
    let const_ptr_pairs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
