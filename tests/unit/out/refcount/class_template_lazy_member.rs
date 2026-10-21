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
pub struct Point {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_int_ {
    #[offset(0)]
    pub val: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_Point_ {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_int_ {
    #[offset(0)]
    pub val: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_Point_ {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let i: Value<Box_int_> = Rc::new(RefCell::new(Box_int_ { val: 3_() }));
    unimplemented!();
    let p: Value<Box_Point_> = Rc::new(RefCell::new(Box_Point_ {
        val: Point { x: 4_() },
    }));
    unimplemented!();
    let d: Value<Deduced_int_> = Rc::new(RefCell::new(Deduced_int_ { val: 5_() }));
    unimplemented!();
    let dp: Value<Deduced_Point_> = Rc::new(RefCell::new(Deduced_Point_ {
        val: Point { x: 6_() },
    }));
    unimplemented!();
    return unimplemented!();
}
pub trait Box_Point_Impl {
    fn get(&self) -> Point;
    fn twice(&self) -> Point {
        unimplemented!()
    }
}
impl Box_Point_Impl for Ptr<Box_Point_> {
    fn get(&self) -> Point {
        return (*self).with(|__s| __s.val.clone());
    }
}
pub trait Box_int_Impl {
    fn get(&self) -> () {
        unimplemented!()
    }
    fn twice(&self) -> ();
}
impl Box_int_Impl for Ptr<Box_int_> {
    fn twice(&self) -> () {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub trait Deduced_Point_Impl {
    fn get(&self) -> Point;
}
impl Deduced_Point_Impl for Ptr<Deduced_Point_> {
    fn get(&self) -> Point {
        return (*self).with(|__s| __s.val.clone());
    }
}
pub trait Deduced_int_Impl {
    fn twice(&self) -> ();
}
impl Deduced_int_Impl for Ptr<Deduced_int_> {
    fn twice(&self) -> () {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub fn __cpp2rust_init_globals() {}
