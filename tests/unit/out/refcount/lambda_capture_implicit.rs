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
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let b: Value<()> = Rc::new(RefCell::new(2_()));
    let c: Value<()> = Rc::new(RefCell::new(3_()));
    let by_value: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let a: Value<()> = Rc::new(RefCell::new((*a.borrow())));
            let b: Value<()> = Rc::new(RefCell::new((*b.borrow())));
            let c: Value<()> = Rc::new(RefCell::new((*c.borrow())));
        },
        |mut x: ()| -> () {
            return ((((*a.borrow()) + (*b.borrow())) + (*c.borrow())) + x);
        }
    )));
    unimplemented!();
    (*a.borrow_mut()) = 100_();
    unimplemented!();
    let by_ref: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let a: Ptr<()> = a.as_pointer();
            let b: Ptr<()> = b.as_pointer();
            let c: Ptr<()> = c.as_pointer();
        },
        |mut x: ()| -> () {
            return ((((a.read()) + (b.read())) + (c.read())) + x);
        }
    )));
    unimplemented!();
    (*b.borrow_mut()) = 200_();
    unimplemented!();
    let mixed: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let c: Ptr<()> = c.as_pointer();
            let a: Value<()> = Rc::new(RefCell::new((*a.borrow())));
            let b: Value<()> = Rc::new(RefCell::new((*b.borrow())));
        },
        |mut x: ()| -> () {
            {
                let __rhs = x;
                c.with_mut(|__v| *__v = *__v + __rhs)
            };
            return (((*a.borrow()) + (*b.borrow())) + (c.read()));
        }
    )));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
