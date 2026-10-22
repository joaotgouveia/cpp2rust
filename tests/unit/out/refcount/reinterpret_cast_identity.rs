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
    let val: Value<u32> = Rc::new(RefCell::new(42_u32));
    let mut original: Ptr<u32> = (val.as_pointer());
    let mut as_u16: Ptr<u16> = original.reinterpret_cast::<u16>();
    let mut back: Ptr<u32> = as_u16.reinterpret_cast::<u32>();
    unimplemented!();
    unimplemented!();
    let mut as_u8: Ptr<u8> = original.reinterpret_cast::<u8>();
    let mut back2: Ptr<u32> = as_u8.reinterpret_cast::<u32>();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
