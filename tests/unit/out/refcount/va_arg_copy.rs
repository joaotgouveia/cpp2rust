extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_with_copy_0(count: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<()> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    let aq: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    (*aq.borrow_mut()) = (*ap.borrow_mut()).clone();
    let mut sum1: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < (*count.borrow())) as ()) != 0) {
        sum1 += (*ap.borrow_mut()).arg::<()>();
        i.postfix_inc();
    }
    let mut sum2: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < (*count.borrow())) as ()) != 0) {
        sum2 += (*aq.borrow_mut()).arg::<()>();
        i.postfix_inc();
    }
    unimplemented!();
    return (sum1 + sum2);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
