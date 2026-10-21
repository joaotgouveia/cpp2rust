extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inner_0(mut count: (), ap: Ptr<VaArg>) -> () {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < count) as ()) != 0) {
        total += (*ap.borrow_mut()).arg::<()>();
        i.postfix_inc();
    }
    return total;
}
pub fn outer_1(count: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<()> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: () = ({ inner_0((*count.borrow()), (*ap.borrow()).clone()) });
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
