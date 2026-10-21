extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut p: Ptr<()>) -> Ptr<()> {
    return (p.offset((5_()) as isize));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p1: Ptr<()> =
        Ptr::alloc_array((0..10_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let mut i: () = unimplemented!();
    'loop_: while (i < 10_()) {
        elem!(p1, i).write({ i });
        i.prefix_inc();
    }
    let mut out: () = (elem!(({ foo_0((p1.offset((1_()) as isize)),) }), 3_()).read());
    p1.delete();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
