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
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let x: Value<i32> = Rc::new(RefCell::new(0));
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let ul: Value<u64> = Rc::new(RefCell::new(0_u64));
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
