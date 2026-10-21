extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(v: Ptr<V>) -> () {
    return v.with(|__s| __s.x);
}
pub fn operator_eq_1(a: Ptr<V>, b: Ptr<V>) -> () {
    return ({ a.with(|__s| __s.x) } == { b.with(|__s| __s.x) });
}
pub fn scaled_2(v: Ptr<V>, mut k: ()) -> () {
    return ({ v.with(|__s| __s.x) } * { k });
}
pub fn scaled_3(v: Ptr<V>, mut k: ()) -> () {
    return ({ v.with(|__s| __s.x) } * { k });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct V {
    #[offset(0)]
    pub x: (),
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(V { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(V { x: other.x.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for V {}
pub fn get_4(w: Ptr<W_int_>) -> () {
    return w.with(|__s| __s.x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct W_int_ {
    #[offset(0)]
    pub x: (),
}
pub fn get_5(w: Ptr<W_long_>) -> () {
    return w.with(|__s| __s.x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct W_long_ {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub x: (),
}
pub fn declared_then_defined_6(d: Ptr<D>) -> () {
    return (d.with(|__s| __s.x) + 1_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<V> = Rc::new(RefCell::new(V { x: 3_() }));
    let b: Value<V> = Rc::new(RefCell::new(V { x: 3_() }));
    let c: Value<V> = Rc::new(RefCell::new(V { x: 4_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let wi: Value<W_int_> = Rc::new(RefCell::new(W_int_ { x: 5_() }));
    let wl: Value<W_long_> = Rc::new(RefCell::new(W_long_ { x: 6_() }));
    unimplemented!();
    unimplemented!();
    let d: Value<D> = Rc::new(RefCell::new(D { x: 7_() }));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
