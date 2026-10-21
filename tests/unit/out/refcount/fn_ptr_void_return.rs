extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn negate_0(mut x: Ptr<()>) {
    x.write({ -(x.read()) });
}
pub fn zero_out_1(mut x: Ptr<()>) {
    x.write(0_());
}
pub fn run_2(mut fn_: FnPtr<fn(Ptr<()>)>, mut x: Ptr<()>) {
    ({ fn_.call((x).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<()> = Rc::new(RefCell::new(42_()));
    ({ run_2(FnPtr::<fn(Ptr<()>)>::new(negate_0), (a.as_pointer())) });
    unimplemented!();
    ({ run_2(FnPtr::<fn(Ptr<()>)>::new(zero_out_1), (a.as_pointer())) });
    unimplemented!();
    let mut fn_: FnPtr<fn(Ptr<()>)> = FnPtr::<fn(Ptr<()>)>::new(negate_0);
    unimplemented!();
    let b: Value<()> = Rc::new(RefCell::new(10_()));
    ({ fn_.call((b.as_pointer())) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
