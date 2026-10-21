extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct widget {
    #[offset(0)]
    pub id: (),
}
pub fn a_value_0() -> () {
    let mut w: widget = <widget>::default();
    w.id = 11_();
    return w.id;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub type widget_enum = ();
pub const widget_enum_WIDGET_A: widget_enum = 0;
pub const widget_enum_WIDGET_B: widget_enum = 1;
pub const widget_enum_WIDGET_C: widget_enum = 2;
pub fn b_value_1() -> () {
    let mut w: widget_enum = widget_enum_WIDGET_C;
    return (w as ());
}
pub fn __cpp2rust_init_globals() {}
