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
    let storage: Value<()> = Rc::new(RefCell::new(7_()));
    let mut p: Ptr<()> = (storage.as_pointer());
    let mut np: Ptr<()> = Ptr::<()>::null();
    if !(p).is_null() {
        unimplemented!();
    }
    if !(!(p).is_null()) {
        unimplemented!();
    }
    if !(np).is_null() {
        unimplemented!();
    }
    if !(!(np).is_null()) {
        unimplemented!();
    }
    let mut iter: Ptr<()> = (p).clone();
    let mut iters: () = unimplemented!();
    'loop_: while !(iter).is_null() {
        iters.prefix_inc();
        iter = Ptr::<()>::null();
    }
    unimplemented!();
    let mut t3: () = if !(p).is_null() {
        1_()
    } else {
        unimplemented!()
    };
    unimplemented!();
    let mut t4: () = if !(np).is_null() {
        1_()
    } else {
        unimplemented!()
    };
    unimplemented!();
    let mut t5: () = (!(!(p).is_null()) as ());
    unimplemented!();
    let mut t6: () = (!(!(np).is_null()) as ());
    unimplemented!();
    let mut b2: () = !(p).is_null();
    let mut b3: () = !(np).is_null();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
