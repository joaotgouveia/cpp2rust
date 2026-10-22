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
pub struct NonCopy {
    #[offset(0)]
    #[byte_size(24)]
    pub data: (),
    #[offset(24)]
    pub tag: (),
}
impl Default for NonCopy {
    fn default() -> Self {
        {
            NonCopy {
                data: unimplemented!(),
                tag: 0_(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut arr: [NonCopy; 3] = std::array::from_fn::<_, 3, _>(|_| <NonCopy>::default());
    arr[(0_()) as usize].tag = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
