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
    let b: Value<()> = Rc::new(RefCell::new(2_()));
    let b_ptr: Value<Ptr<()>> = Rc::new(RefCell::new((b.as_pointer())));
    (*b_ptr.borrow()).write(3_());
    let mut b_ptr_ptr: Ptr<Ptr<()>> = (b_ptr.as_pointer());
    (b_ptr_ptr.read()).write(4_());
    (*b_ptr.borrow()).write({ ((b_ptr_ptr.read()).read()) });
    let mut offset: () = ((*b_ptr.borrow()).clone() - (*b_ptr.borrow()).clone()) as ();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
