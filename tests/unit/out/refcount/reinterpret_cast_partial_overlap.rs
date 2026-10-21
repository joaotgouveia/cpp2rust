extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([67305985_(), 134678021_()])));
    let mut bytes: Ptr<()> = (arr.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    elem!(bytes, 3_()).write(255_());
    unimplemented!();
    elem!(bytes, 4_()).write(170_());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
