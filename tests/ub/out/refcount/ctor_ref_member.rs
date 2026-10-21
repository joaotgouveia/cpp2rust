extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub r: Ptr<()>,
}
impl S {
    pub fn new(x: Ptr<()>) -> Self {
        Self { r: (x).clone() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s: S = {
        let __tmp_0: Value<()> = Rc::new(RefCell::new(5_()));
        S::new({ __tmp_0.as_pointer() })
    };
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
