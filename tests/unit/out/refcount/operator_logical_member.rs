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
pub struct S {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
    let f: Value<S> = Rc::new(RefCell::new(S { v: 0_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn operator_not(&self) -> ();
    fn operator_and(&self, o: Ptr<S>) -> ();
    fn operator_or(&self, o: Ptr<S>) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_not(&self) -> () {
        return ((*self).with(|__s| __s.v) == 0_());
    }
    fn operator_and(&self, o: Ptr<S>) -> () {
        return ((*self).with(|__s| __s.v) != 0_()) && (o.with(|__s| __s.v) != 0_());
    }
    fn operator_or(&self, o: Ptr<S>) -> () {
        return ((*self).with(|__s| __s.v) != 0_()) || (o.with(|__s| __s.v) != 0_());
    }
}
pub fn __cpp2rust_init_globals() {}
