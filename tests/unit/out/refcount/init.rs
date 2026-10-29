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
pub struct X {
    #[offset(0)]
    pub x: (),
}
pub fn func_0() -> () {
    return 42_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut p: Ptr<()> = Ptr::<()>::null();
    let g: Ptr<()> = x.as_pointer();
    let mut q: Ptr<()> = (x.as_pointer());
    let mut z: Ptr<()> = (p).clone();
    let xx: Value<X> = Rc::new(RefCell::new(<X>::default()));
    let mut zz: Ptr<X> = (xx.as_pointer());
    (*xx.borrow_mut()).x = 1_();
    q = (field_ptr!(xx.as_pointer(), x));
    q = (field_ptr!(zz, x));
    field!(zz, x).write(2_());
    let mut ww: X = (*xx.borrow()).clone();
    ww = (*xx.borrow()).clone();
    let mut aa: () = ({ func_0() });
    aa = ({ func_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
