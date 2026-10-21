extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_int_0(mut a: (), mut b: ()) -> () {
    let mut x: () = (a.reinterpret_cast::<()>().read());
    let mut y: () = (b.reinterpret_cast::<()>().read());
    return (((x > y) as ()) - ((x < y) as ()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        5_(),
        2_(),
        9_(),
        1_(),
        7_(),
        3_(),
        8_(),
        4_(),
    ])));
    unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < 7_()) as ()) != 0) {
        unimplemented!();
        i.prefix_inc();
    }
    let key: Value<()> = Rc::new(RefCell::new(7_()));
    let mut hit: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    let miss_key: Value<()> = Rc::new(RefCell::new(42_()));
    let mut miss: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
