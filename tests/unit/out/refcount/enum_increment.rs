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
pub const color_COLOR_LAST: color = 3;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut count: () = 0_();
    let mut c: color = color_RED;
    'loop_: while ((((c as ()) < (color_COLOR_LAST as i32)) as ()) != 0) {
        count.postfix_inc();
        c.postfix_inc();
    }
    unimplemented!();
    let mut c: color = color_RED;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
