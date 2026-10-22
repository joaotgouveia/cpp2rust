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
    let a: Value<()> = Rc::new(RefCell::new(10_()));
    let b: Value<()> = Rc::new(RefCell::new(20_()));
    let pa: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer())));
    let pb: Value<Ptr<()>> = Rc::new(RefCell::new((b.as_pointer())));
    let mut ppa: Ptr<Ptr<()>> = (pa.as_pointer());
    let mut ppb: Ptr<Ptr<()>> = (pb.as_pointer());
    let mut r1: () = (unimplemented!().read());
    let mut r2: () = (unimplemented!().read());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
