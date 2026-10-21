extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct pair {
    #[offset(0)]
    #[byte_size(16)]
    pub a: Box<[()]>,
    #[offset(16)]
    #[byte_size(16)]
    pub b: Box<[()]>,
}
impl Default for pair {
    fn default() -> Self {
        {
            pair {
                a: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                b: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<pair> = <Value<pair>>::default();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
