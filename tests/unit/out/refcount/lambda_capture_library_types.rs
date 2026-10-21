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
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        || -> () {
            return unimplemented!();
        }
    )));
    let g: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*f.borrow()).copy_from()));
    let h: Value<()> = Rc::new(RefCell::new((*unimplemented!()).move_from()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let u: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        || -> () {
            return if !(unimplemented!()).is_null() {
                (unimplemented!().read())
            } else {
                unimplemented!()
            };
        }
    )));
    let q: Value<()> = Rc::new(RefCell::new((*unimplemented!()).move_from()));
    unimplemented!();
    unimplemented!();
    let n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let inner: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<()> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> () {
            return (*n.borrow_mut()).prefix_inc();
        }
    )));
    let outer: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let inner: Value<FnPtr<fn() -> ()>> =
                Rc::new(RefCell::new((*inner.borrow()).copy_from()));
        },
        || -> () {
            return ({ (*inner.borrow()).call() }).clone();
        }
    )));
    unimplemented!();
    let outer2: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*outer.borrow()).copy_from()));
    unimplemented!();
    unimplemented!();
    let outer3: Value<()> = Rc::new(RefCell::new((*unimplemented!()).move_from()));
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
