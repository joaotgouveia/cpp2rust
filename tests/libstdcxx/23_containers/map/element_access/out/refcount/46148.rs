extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(1)]
pub struct moveable {}
impl moveable {
    pub fn move_from(_a0: Ptr<moveable>) -> Self {
        Self {}
    }
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for moveable {
    fn default() -> Self {
        { moveable::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let the_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return ({ moveableImpl::to_(&unimplemented!()) });
}
pub trait moveableImpl {
    fn to_(&self) -> ();
}
impl moveableImpl for Ptr<moveable> {
    fn to_(&self) -> () {
        return 0_();
    }
}
pub fn __cpp2rust_init_globals() {}
