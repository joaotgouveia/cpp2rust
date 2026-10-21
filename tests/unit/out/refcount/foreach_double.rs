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
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut square: () = unimplemented!();
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut e1: () = (unimplemented!().read());
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut e2: () = (unimplemented!().read());
            square += (e1 * e2);
        }
    }
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let e1: Ptr<()> = unimplemented!();
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let e2: Ptr<()> = unimplemented!();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let e1: Ptr<()> = unimplemented!();
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let e2: Ptr<()> = unimplemented!();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let e1: Ptr<()> = unimplemented!();
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let e2: Ptr<()> = unimplemented!();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut v1: () = unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    'loop_: for row in IterRange::new(unimplemented!(), unimplemented!()) {
        let row: Ptr<()> = unimplemented!();
        'loop_: for col in IterRange::new(unimplemented!(), unimplemented!()) {
            let col: Ptr<()> = unimplemented!();
            square += { (col.read()) };
        }
    }
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
