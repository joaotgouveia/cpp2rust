extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static last_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: (),
}
pub fn by_value_2(s: S) -> () {
    let s: Value<S> = Rc::new(RefCell::new(s));
    unimplemented!();
    return { (*s.borrow()).v };
}
pub fn with_default_3(s: Option<S>) -> () {
    let s: Value<S> = Rc::new(RefCell::new(s.unwrap_or_else(|| S { v: 5_() })));
    return { (*s.borrow()).v };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub v: (),
}
impl Holder {
    pub fn new(s: S) -> Self {
        let s: Value<S> = Rc::new(RefCell::new(s));
        Self {
            v: { (*s.borrow()).v },
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    {
        let s: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    unimplemented!();
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    unimplemented!();
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    {
        let h: Value<Holder> = Rc::new(RefCell::new(Holder::new({ S { v: 3_() } })));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn add(&self, s: S) -> ();
}
impl HolderImpl for Ptr<Holder> {
    fn add(&self, s: S) -> () {
        let s: Value<S> = Rc::new(RefCell::new(s));
        return ((*self).with(|__s| __s.v) + { (*s.borrow()).v });
    }
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut()).postfix_inc();
        last_1.with(|rc| *rc.borrow_mut() = (*self).with(|__s| __s.v));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
    let _ = last_1.with(|_| ());
}
