extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: (),
    #[offset(24)]
    pub a: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    unimplemented!();
    'loop_: for e in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut e: () = (unimplemented!().read());
        (*s.borrow_mut()).a.postfix_inc();
    }
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
