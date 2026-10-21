extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_nonnull_0(count: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<()> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: () = -1_();
    let mut i: () = 0_();
    'loop_: while (((i < (*count.borrow())) as ()) != 0) {
        let mut p: Ptr<()> = (*ap.borrow_mut()).arg::<Ptr<()>>();
        if (((!((p).is_null())) as ()) != 0) {
            result = { (p.read()) };
            break;
        }
        i.postfix_inc();
    }
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(42_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
