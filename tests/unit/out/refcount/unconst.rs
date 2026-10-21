extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn set_0(r: Ptr<()>) {
    r.write(7_());
}
pub fn set_through_1(v: Ptr<()>) {
    ({ set_0((v).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let mut p: Ptr<()> = (a.as_pointer());
    let mut q: Ptr<()> = (<()>::from_int((p).to_any().to_int())).reinterpret_cast::<()>();
    unimplemented!();
    let v: Value<()> = Rc::new(RefCell::new(1_()));
    let cr: Ptr<()> = v.as_pointer();
    ({ set_0((cr).clone()) });
    unimplemented!();
    (*v.borrow_mut()) = unimplemented!();
    ({ set_through_1(v.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
