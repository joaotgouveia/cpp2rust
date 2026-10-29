extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn logf_impl_0(mut fmt: Ptr<()>, ap: Ptr<VaArg>) -> () {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    &(fmt);
    return ({ (*ap.borrow_mut()).arg::<()>() } + { (*ap.borrow_mut()).arg::<()>() });
}
pub fn logf_1(fmt: Ptr<()>, __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let fmt: Value<Ptr<()>> = Rc::new(RefCell::new(fmt));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: () = ({ logf_impl_0((*fmt.borrow()).clone(), (*ap.borrow()).clone()) });
    return result;
}
pub fn lenf_2(fmt: Ptr<()>, __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let fmt: Value<Ptr<()>> = Rc::new(RefCell::new(fmt));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut s: Ptr<()> = (*ap.borrow_mut()).arg::<Ptr<()>>();
    let mut result: () = (unimplemented!() as ());
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut dummy: Ptr<()> = Ptr::<()>::from_string_literal(b"dummy");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
