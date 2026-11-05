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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((b.as_pointer())));
    cpp_swap!(p.as_pointer(), q.as_pointer());
    assert!(
        ({
            ({ (*p.borrow()).clone() } == { (b.as_pointer()) })
                && ({ (*q.borrow()).clone() } == { (a.as_pointer()) })
        })
    );
    assert!(({ ((*a.borrow()) == 1) && ((*b.borrow()) == 2) }));
    let n: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    cpp_swap!(n.as_pointer(), p.as_pointer());
    assert!(({ ({ (*n.borrow()).clone() } == { (b.as_pointer()) }) && ((*p.borrow()).is_null()) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
