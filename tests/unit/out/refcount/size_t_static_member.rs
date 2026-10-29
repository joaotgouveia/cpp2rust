extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static c_0: Value<()> = Rc::new(RefCell::new(5_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
thread_local!(
    pub static table_size_1: Value<()> = Rc::new(RefCell::new(256_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Table_char_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut pa: Ptr<()> = (c_0.with(|v| v.as_pointer()));
    unimplemented!();
    let mut G: Ptr<()> = (table_size_1.with(|v| v.as_pointer()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = c_0.with(|_| ());
    let _ = table_size_1.with(|_| ());
}
