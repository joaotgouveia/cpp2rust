extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut b: [Ptr<i32>; 3] = [
        ((a.as_pointer() as Ptr<i32>).offset(2)),
        ((a.as_pointer() as Ptr<i32>).offset(0)),
        ((a.as_pointer() as Ptr<i32>).offset(1)),
    ];
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    0
}
pub fn __cpp2rust_init_globals() {}
