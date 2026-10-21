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
    let start: Value<()> = Rc::new(RefCell::new(5_()));
    let next: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let start: Value<()> = Rc::new(RefCell::new((*start.borrow())));
        },
        || -> () {
            return (*start.borrow_mut()).postfix_inc();
        }
    )));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let total: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let accumulate: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let total: Value<()> = Rc::new(RefCell::new((*total.borrow())));
        },
        |mut x: ()| -> () {
            (*total.borrow_mut()) += x;
            return (*total.borrow());
        }
    )));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
