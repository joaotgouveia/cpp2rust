extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut s: Ptr<()>, mut n: ()) -> () {
    return if ((s.read()) != 0) {
        ({ strlen_0(s.offset((1_()) as isize), (n + 1_())) })
    } else {
        n
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('s' as ()),
        ('t' as ()),
        ('r' as ()),
        ('\0' as ()),
    ])));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
