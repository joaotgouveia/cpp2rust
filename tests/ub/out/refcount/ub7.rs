extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut s: Ptr<()>) -> () {
    let mut begin: Ptr<()> = (s).clone();
    'loop_: while ((s.read()) != 0) {
        s.prefix_inc();
    }
    return (s - begin) as ();
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
        ('i' as ()),
        ('n' as ()),
        ('g' as ()),
    ])));
    return ({ strlen_0(((s.as_pointer() as Ptr<()>).offset(unimplemented!()))) });
}
pub fn __cpp2rust_init_globals() {}
