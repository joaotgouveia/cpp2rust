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
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut str: Ptr<()> = unimplemented!();
    let limit_l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut r: () = unimplemented!();
    let mut cp: Ptr<()> = Ptr::alloc(Default::default());
    cp.delete();
    let mut b: () = unimplemented!();
    let ilisti: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::has_single_bit(256u)");;
    const _: () = assert!((201411_() >= 201411_()), "__cpp_lib_void_t >= 201411L");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
