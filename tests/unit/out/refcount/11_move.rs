extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn change_0(n: Ptr<()>) {
    let mut m: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ change_0(n.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
