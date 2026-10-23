extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let b: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        ((a.as_pointer() as Ptr<()>).offset(2_())),
        ((a.as_pointer() as Ptr<()>).offset(0_())),
        ((a.as_pointer() as Ptr<()>).offset(1_())),
    ])));
    let c : Value<std_ranges_subrange_const_int_ptrconst_ptr__const_int_ptrconst_ptr__std_ranges_subrange_kind_sized_ > = Rc::new(RefCell::new(std_ranges_subrange_const_int_ptrconst_ptr__const_int_ptrconst_ptr__std_ranges_subrange_kind_sized_ :: new_1 ( { b .as_pointer()  } , )  )) ;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    0
}
pub fn __cpp2rust_init_globals() {}
