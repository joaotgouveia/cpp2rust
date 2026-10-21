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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct T {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 65_() }));
    let mut a: () = ({ SImpl::to__1(&s.as_pointer()) });
    let mut b: () = ({ SImpl::to__2(&s.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let t: Value<T> = Rc::new(RefCell::new(T { v: 3_() }));
    let mut c: () = ({ TImpl::to__1(&t.as_pointer()) });
    let mut d: () = ({ TImpl::to__2(&t.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn to__1(&self) -> ();
    fn to__2(&self) -> ();
}
impl SImpl for Ptr<S> {
    fn to__1(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
    fn to__2(&self) -> () {
        return ((*self).with(|__s| __s.v) + 1_());
    }
}
pub trait TImpl {
    fn to__1(&self) -> ();
    fn to__2(&self) -> ();
}
impl TImpl for Ptr<T> {
    fn to__1(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
    fn to__2(&self) -> () {
        return ((*self).with(|__s| __s.v) + 1_());
    }
}
pub fn __cpp2rust_init_globals() {}
