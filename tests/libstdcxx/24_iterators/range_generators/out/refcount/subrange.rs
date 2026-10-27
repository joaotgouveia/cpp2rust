extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn yield_vector_0() -> () {
    {
        let foo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: () = 123_();
        unimplemented!();
        unimplemented!();
        x = 456_();
        unimplemented!();
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let __tmp_0: Value<()> = Rc::new(RefCell::new(({ yield_vector_0() })));
    let __range1: Ptr<()> = __tmp_0.as_pointer();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (unimplemented!().read());
        unimplemented!();
    }
    0
}
pub fn __cpp2rust_init_globals() {}
