extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn sum_0(mut p: Point) -> () {
    return (p.x + p.y);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p: () = unimplemented!();
    unimplemented!().x += 10_();
    let __rhs = ({ unimplemented!().x } + { unimplemented!().y });
    unimplemented!().y = __rhs;
    let mut s: () = ({ sum_0((*unimplemented!().upgrade().deref()).clone()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
