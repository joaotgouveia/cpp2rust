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
pub struct value_type {
    #[offset(0)]
    pub i: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let o: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "o->i == 51");;
    const _: () = assert!(true, "o->i == (*o).i");;
    const _: () = assert!(true, "&o->i == &(*o).i");;
    0
}
pub trait value_typeImpl {
    fn operator_addr(&self) -> ();
}
impl value_typeImpl for Ptr<value_type> {
    fn operator_addr(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
