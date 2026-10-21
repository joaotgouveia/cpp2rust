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
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let mut p: Ptr<()> = (x.as_pointer());
    {
        p.with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    p = ((a.as_pointer() as Ptr<()>).offset(1_()));
    {
        p.with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    p.prefix_dec();
    {
        p.with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    p = (x.as_pointer());
    {
        p.with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    let mut p2: Ptr<()> = (p).clone();
    {
        p2.with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
