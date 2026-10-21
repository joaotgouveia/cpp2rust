extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn read_0(v: Ptr<()>) -> () {
    return (v.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let factor: Value<()> = Rc::new(RefCell::new(3_()));
    let scale: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let factor: Value<()> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: ()| -> () {
            return (x * (*factor.borrow()));
        }
    )));
    unimplemented!();
    (*factor.borrow_mut()) = 100_();
    unimplemented!();
    let slot: Value<()> = Rc::new(RefCell::new(7_()));
    let p: Value<Ptr<()>> = Rc::new(RefCell::new((slot.as_pointer())));
    let read_ptr: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let p: Value<Ptr<()>> = Rc::new(RefCell::new((*p.borrow()).clone()));
        },
        || -> () {
            return ((*p.borrow()).read());
        }
    )));
    (*slot.borrow_mut()) = 8_();
    unimplemented!();
    let s: Value<S> = Rc::new(RefCell::new(S { x: 1_(), y: 2_() }));
    let sum: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let s: Value<S> = Rc::new(RefCell::new((*s.borrow()).clone()));
        },
        || -> () {
            return ({ (*s.borrow()).x } + { (*s.borrow()).y });
        }
    )));
    (*s.borrow_mut()).x = 50_();
    unimplemented!();
    let mut base: () = 10_();
    let shifted: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let y: Value<()> = Rc::new(RefCell::new((base + 1_())));
        },
        |mut x: ()| -> () {
            return (x + (*y.borrow()));
        }
    )));
    unimplemented!();
    let k: Value<()> = Rc::new(RefCell::new(3_()));
    let by_copy: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<()> = Rc::new(RefCell::new((*k.borrow())));
        },
        |mut x: ()| -> () {
            return (x + 3_());
        }
    )));
    unimplemented!();
    let by_copy_used: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<()> = Rc::new(RefCell::new((*k.borrow())));
        },
        |mut x: ()| -> () {
            return (x + ({ read_0(k.as_pointer()) }));
        }
    )));
    unimplemented!();
    let implicit_used: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<()> = Rc::new(RefCell::new((*k.borrow())));
        },
        |mut x: ()| -> () {
            return (x + ({ read_0(k.as_pointer()) }));
        }
    )));
    unimplemented!();
    let by_ref: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Ptr<()> = k.as_pointer();
        },
        |mut x: ()| -> () {
            return (x + 3_());
        }
    )));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
