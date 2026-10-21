extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    static inner_const_0: Value<()> = Rc::new(RefCell::new(1_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
thread_local!(
    pub static inner_const_1: Value<()> = Rc::new(RefCell::new(2_()));
);
pub type anon_3 = ();
pub const anon_3_kValue: anon_3 = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn f() -> () {
        return counter_2.with(|rc| *rc.borrow());
    }
}
thread_local!(
    pub static counter_2: Value<()> = Rc::new(RefCell::new(10_()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let c: Value<C> = Rc::new(RefCell::new(<C>::default()));
    unimplemented!();
    unimplemented!();
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    let mut p: Ptr<S> = (s.as_pointer());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*counter_2.with(Value::clone).borrow_mut()) = 20_();
    unimplemented!();
    (*counter_2.with(Value::clone).borrow_mut()) += 5_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait CImpl {
    fn get(&self) -> ();
}
impl CImpl for Ptr<C> {
    fn get(&self) -> () {
        return inner_const_0.with(|rc| *rc.borrow());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = inner_const_0.with(|_| ());
    let _ = inner_const_1.with(|_| ());
    let _ = counter_2.with(|_| ());
}
