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
pub struct point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let src: Value<point> = Rc::new(RefCell::new(point { x: 3_(), y: 7_() }));
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    let dst: Value<point> = <Value<point>>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
