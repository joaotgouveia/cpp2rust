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
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: () = 5_();
    let mut v5: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(10_());
    unimplemented!().write(20_());
    unimplemented!().write(30_());
    let v6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
