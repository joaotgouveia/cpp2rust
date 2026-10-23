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
pub struct Inner {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub inner: (),
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            inner: unimplemented!(),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut o: () = unimplemented!();
    unimplemented!().x += 5_();
    let mut sum: () = ({ unimplemented!().x } + { unimplemented!().y });
    let mut a: () = unimplemented!();
    let mut b: () = unimplemented!();
    let __rhs = (unimplemented!().read());
    unimplemented!().write(__rhs);
    unimplemented!();
    return 0_();
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
