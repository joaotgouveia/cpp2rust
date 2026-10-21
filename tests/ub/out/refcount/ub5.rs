extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn null_0(mut p: Ptr<Ptr<()>>) {
    p.write(Ptr::<()>::null());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let p: Value<Ptr<()>> = Rc::new(RefCell::new((x.as_pointer())));
    ({ null_0((p.as_pointer())) });
    let r: Ptr<()> = (*p.borrow()).clone();
    return (r.read());
}
pub fn __cpp2rust_init_globals() {}
