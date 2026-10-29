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
    let a: Value<std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___> =
        Rc::new(RefCell::new(
            <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default(),
        ));
    (*a.borrow_mut())[1_()] = 10_();
    let b: Value<std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___> =
        Rc::new(RefCell::new((*a.borrow()).clone()));
    (*b.borrow_mut())[1_()] = 20_();
    unimplemented!();
    let x: Value<std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___> =
        Rc::new(RefCell::new(
            <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default(),
        ));
    (*x.borrow_mut())[2_()] = 3_();
    let y: Value<std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___> =
        Rc::new(RefCell::new((*x.borrow()).clone()));
    (*y.borrow_mut())[2_()] = 4_();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
