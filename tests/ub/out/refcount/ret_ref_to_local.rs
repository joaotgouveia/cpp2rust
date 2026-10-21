extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> Ptr<()> {
    let __tmp_0: Value<()> = Rc::new(RefCell::new(5_()));
    return __tmp_0.as_pointer();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut bar: () = (({ foo_0() }).read());
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
