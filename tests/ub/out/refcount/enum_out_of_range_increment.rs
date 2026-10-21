extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type color = ();
pub const color_RED: color = 0;
pub const color_GREEN: color = 1;
pub const color_BLUE: color = 2;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut c: color = color_BLUE;
    c.postfix_inc();
    return if ((((c as ()) == (color_RED as i32)) as ()) != 0) {
        unimplemented!()
    } else {
        1_()
    };
}
pub fn __cpp2rust_init_globals() {}
