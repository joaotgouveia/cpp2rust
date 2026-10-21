extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_nth_0(mut n: (), ap: Ptr<VaArg>) -> () {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    let mut i: () = unimplemented!();
    'loop_: while (((i < n) as ()) != 0) {
        (*ap.borrow_mut()).arg::<()>();
        i.postfix_inc();
    }
    return (*ap.borrow_mut()).arg::<()>();
}
pub fn middle_layer_1(mut n: (), ap: Ptr<VaArg>) -> () {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    return ({ extract_nth_0(n, (*ap.borrow()).clone()) });
}
pub fn top_level_2(n: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let n: Value<()> = Rc::new(RefCell::new(n));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: () = ({ middle_layer_1((*n.borrow()), (*ap.borrow()).clone()) });
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
