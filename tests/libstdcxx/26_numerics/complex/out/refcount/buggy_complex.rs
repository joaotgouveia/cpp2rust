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
fn main_0() -> i32 {
    let a: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    let b: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    let c: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*(*a.borrow()).upgrade().deref()).clone();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
