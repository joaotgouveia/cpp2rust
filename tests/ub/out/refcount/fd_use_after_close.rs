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
    let mut fd: () = unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut n: () = unimplemented!();
    return if (((n == -1_()) as ()) != 0) {
        0_()
    } else {
        1_()
    };
}
pub fn __cpp2rust_init_globals() {}
