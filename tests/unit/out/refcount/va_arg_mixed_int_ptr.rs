extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_args_0(count: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<()> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < (*count.borrow())) as ()) != 0) {
        let mut tag: () = (*ap.borrow_mut()).arg::<()>();
        if (((tag == unimplemented!()) as ()) != 0) {
            total += (*ap.borrow_mut()).arg::<()>();
        } else {
            let mut ptr: Ptr<()> = (*ap.borrow_mut()).arg::<Ptr<()>>();
            total += { (ptr.read()) };
        }
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(100_()));
    unimplemented!();
    let y: Value<()> = Rc::new(RefCell::new(50_()));
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
