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
pub struct S_2_ {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub w: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_1_ {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_3_ {
    #[offset(0)]
    pub v: (),
}
pub fn scale_0(mut x: ()) -> () {
    return (x * 1_());
}
pub fn scale_1(mut x: ()) -> () {
    return (x * 3_());
}
pub fn scale_2(mut x: ()) -> () {
    return (x - 2_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s1: Value<S_1_> = Rc::new(RefCell::new(S_1_ { v: 5_() }));
    let s2: Value<S_2_> = Rc::new(RefCell::new(S_2_ { v: 5_(), w: 1_() }));
    let s3: Value<S_3_> = Rc::new(RefCell::new(S_3_ { v: 5_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait S_1_Impl {
    fn get(&self) -> ();
}
impl S_1_Impl for Ptr<S_1_> {
    fn get(&self) -> () {
        return ((*self).with(|__s| __s.v) * 1_());
    }
}
pub trait S_2_Impl {
    fn get(&self) -> ();
}
impl S_2_Impl for Ptr<S_2_> {
    fn get(&self) -> () {
        return ((*self).with(|__s| __s.v) - (*self).with(|__s| __s.w));
    }
}
pub trait S_3_Impl {
    fn get(&self) -> ();
}
impl S_3_Impl for Ptr<S_3_> {
    fn get(&self) -> () {
        return ((*self).with(|__s| __s.v) * 3_());
    }
}
pub fn __cpp2rust_init_globals() {}
