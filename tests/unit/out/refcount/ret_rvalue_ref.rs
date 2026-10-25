extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(v: Ptr<()>) -> () {
    return (v.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let __tmp_0: Value<()> = Rc::new(RefCell::new(5_()));
    let i2: Ptr<()> = __tmp_0.as_pointer();
    unimplemented!();
    let mut i3: () = ({ foo_0((i2).clone()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
