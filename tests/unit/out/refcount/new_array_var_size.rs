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
    let N: Value<()> = Rc::new(RefCell::new(5_()));
    let mut A: Ptr<()> = Ptr::alloc_array(
        (0..(*N.borrow()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    A.delete();
    let N2: Ptr<()> = N.as_pointer();
    let mut A2: Ptr<()> = Ptr::alloc_array(
        (0..(N2.read()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    A2.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
