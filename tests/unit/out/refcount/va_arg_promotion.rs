extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_promotions_0(count: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<()> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut a: () = (*ap.borrow_mut()).arg::<()>();
    let mut b: () = (*ap.borrow_mut()).arg::<()>();
    let mut c: () = (*ap.borrow_mut()).arg::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return ((a + b) + (c as ()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: () = ('A' as ());
    let mut y: () = 10_();
    let mut z: () = 3_f32;
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
