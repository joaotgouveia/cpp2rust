extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_then_product_0(first: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let first: Value<()> = Rc::new(RefCell::new(first));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    let mut sum: () = (*first.borrow());
    let mut product: () = (*first.borrow());
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut val: () = unimplemented!();
    'loop_: while (((({
        val = (*ap.borrow_mut()).arg::<()>();
        val
    }) != 0_()) as ())
        != 0)
    {
        sum += val;
    }
    (*ap.borrow_mut()) = __args.as_pointer();
    'loop_: while (((({
        val = (*ap.borrow_mut()).arg::<()>();
        val
    }) != 0_()) as ())
        != 0)
    {
        product *= val;
    }
    return (sum + product);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
