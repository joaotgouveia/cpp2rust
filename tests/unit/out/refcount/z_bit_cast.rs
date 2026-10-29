extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn decay_cast_0(mut a1: Ptr<()>) {}
pub fn bit_cast_1(mut p: ()) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    ({ decay_cast_0((a1.as_pointer() as Ptr<()>)) });
    ({ decay_cast_0(((a1.as_pointer() as Ptr<()>).offset(0_()))) });
    ({ bit_cast_1(((a1.as_pointer() as Ptr<()>) as Ptr<()>).to_any()) });
    ({ bit_cast_1((((a1.as_pointer() as Ptr<()>).offset(0_())) as Ptr<()>).to_any()) });
    ({ bit_cast_1(((a1.as_pointer()) as Ptr<()>).to_any()) });
    let mut ptr: () = ((a1.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
