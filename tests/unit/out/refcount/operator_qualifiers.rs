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
    let s: Value<S> = Rc::new(RefCell::new(S { v: 10_() }));
    let cs: Value<S> = Rc::new(RefCell::new(S { v: 10_() }));
    let vs: Value<S> = Rc::new(RefCell::new(S { v: 10_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn operator_add_1(&self, a: ()) -> ();
    fn operator_add_2(&self, a: ()) -> ();
    fn operator_add_3(&self, a: ()) -> ();
    fn operator_sub_4(&self, a: ()) -> ();
    fn operator_sub_5(&self, a: ()) -> ();
    fn operator_mul_6(&self, a: ()) -> ();
    fn operator_mul_7(&self, a: ()) -> ();
    fn operator_index_8(&self, i: ()) -> ();
    fn operator_index_9(&self, i: ()) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, mut a: ()) -> () {
        return ((*self).with(|__s| __s.v) + a);
    }
    fn operator_add_2(&self, mut a: ()) -> () {
        return (((*self).with(|__s| __s.v) + a) + 1_());
    }
    fn operator_add_3(&self, mut a: ()) -> () {
        return (((*self).with(|__s| __s.v) + a) + 2_());
    }
    fn operator_sub_4(&self, mut a: ()) -> () {
        return ((*self).with(|__s| __s.v) - a);
    }
    fn operator_sub_5(&self, mut a: ()) -> () {
        return (((*self).with(|__s| __s.v) - a) - 1_());
    }
    fn operator_mul_6(&self, mut a: ()) -> () {
        return ((*self).with(|__s| __s.v) * a);
    }
    fn operator_mul_7(&self, mut a: ()) -> () {
        return (((*self).with(|__s| __s.v) * a) * 2_());
    }
    fn operator_index_8(&self, mut i: ()) -> () {
        return ((*self).with(|__s| __s.v) + i);
    }
    fn operator_index_9(&self, mut i: ()) -> () {
        return (((*self).with(|__s| __s.v) + i) + 100_());
    }
}
pub fn __cpp2rust_init_globals() {}
