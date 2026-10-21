extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_ints_0(first: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let first: Value<()> = Rc::new(RefCell::new(first));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    let mut total: () = (*first.borrow());
    (*args.borrow_mut()) = __args.as_pointer();
    let mut val: () = unimplemented!();
    'loop_: while (((({
        val = (*args.borrow_mut()).arg::<()>();
        val
    }) != unimplemented!()) as ())
        != 0)
    {
        total += val;
    }
    return total;
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
