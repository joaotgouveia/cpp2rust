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
    let val: Value<()> = Rc::new(RefCell::new(42_()));
    let mut original: Ptr<()> = (val.as_pointer());
    let mut as_u16: Ptr<()> = original.reinterpret_cast::<()>();
    let mut back: Ptr<()> = as_u16.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    let mut as_u8: Ptr<()> = original.reinterpret_cast::<()>();
    let mut back2: Ptr<()> = as_u8.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
