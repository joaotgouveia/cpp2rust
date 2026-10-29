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
pub struct Entry {
    #[offset(0)]
    pub bits: (),
    #[offset(2)]
    pub value: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let table: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            bits: 1_(),
            value: 4369_(),
        },
        Entry {
            bits: 2_(),
            value: 8738_(),
        },
        Entry {
            bits: 3_(),
            value: 13107_(),
        },
        Entry {
            bits: 4_(),
            value: 17476_(),
        },
        Entry {
            bits: 0_(),
            value: 0_(),
        },
        Entry {
            bits: 0_(),
            value: 0_(),
        },
        Entry {
            bits: 0_(),
            value: 0_(),
        },
        Entry {
            bits: 0_(),
            value: 0_(),
        },
    ])));
    let mut table_size: () = 4_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
