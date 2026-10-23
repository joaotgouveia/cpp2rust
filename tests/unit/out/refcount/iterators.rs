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
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let c: Ptr<()> = unimplemented!();
        c.with_mut(|__v| __v.prefix_inc());
    }
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let c: Ptr<()> = unimplemented!();
        println!("{}", (c.read()) as u8 as char);
    }
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (unimplemented!().read());
        println!("{}", c as u8 as char);
    }
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: Ptr<()> = (unimplemented!().read());
        println!("{}", (p.read()));
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
