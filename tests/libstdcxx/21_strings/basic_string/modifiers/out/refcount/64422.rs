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
fn main_0() -> i32 {
    let x: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    unimplemented!();
    let test: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"test\0")));
    unimplemented!();
    unimplemented!();
    let best: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"best\0")));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
    unimplemented!();
    let west: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        119 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
    ])));
    unimplemented!();
    unimplemented!();
    let rest: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        114 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
