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
    let a: Ptr<i32> = ({
        let ___a: Value<i32> = Rc::new(RefCell::new(1));
        let ___b: Value<i32> = Rc::new(RefCell::new(2));
        min_0(___a.as_pointer(), ___b.as_pointer())
    });
    let b: Ptr<i32> = ({
        let ___a: Value<i32> = Rc::new(RefCell::new(1));
        let ___b: Value<i32> = Rc::new(RefCell::new(2));
        max_1(___a.as_pointer(), ___b.as_pointer())
    });
    return (({ (a.read()) } == { (b.read()) }) as i32);
}
pub fn __cpp2rust_init_globals() {}
