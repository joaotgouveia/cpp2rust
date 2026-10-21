extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(true, "Small<int>");
const _: () = assert!((::std::mem::size_of::<()>() == 4_()), "sizeof(int) == 4");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Sized {}
pub fn is_small_0() -> () {
    return true;
}
pub fn is_small_1() -> () {
    return false;
}
pub fn has_size_2() -> () {
    return true;
}
pub fn has_size_3() -> () {
    return false;
}
pub fn pick_4(mut x: ()) -> () {
    {
        return 1_();
    }
    return 2_();
}
pub fn pick_5(mut x: ()) -> () {
    {}
    return 2_();
}
pub fn pick_6(mut x: ()) -> () {
    {}
    return 2_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    const _: () = assert!(!(false), "!Small<long>");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait SizedImpl {
    fn size(&self) -> ();
}
impl SizedImpl for Ptr<Sized> {
    fn size(&self) -> () {
        return 4_();
    }
}
pub fn __cpp2rust_init_globals() {}
