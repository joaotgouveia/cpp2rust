extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct D {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut d: Ptr<()> = Ptr::alloc(0_());
    d.write(5_());
    d.delete();
    let mut c: Ptr<()> = Ptr::alloc(3_());
    c.delete();
    ({
        DImpl::operator_call(
            &Rc::new(RefCell::new(<D>::default())).as_pointer(),
            Ptr::alloc(<A>::default()),
        )
    });
    return 0_();
}
pub trait DImpl {
    fn operator_call(&self, ptr: Ptr<A>);
}
impl DImpl for Ptr<D> {
    fn operator_call(&self, mut ptr: Ptr<A>) {
        ptr.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
