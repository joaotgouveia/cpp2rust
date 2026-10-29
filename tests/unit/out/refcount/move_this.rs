extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct Chain {
    #[offset(0)]
    pub v: (),
}
impl Chain {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Chain>) -> Self {
        Self {
            v: (o.with(|__s| __s.v) + 100_()),
        }
    }
    pub fn move_from(o: Ptr<Chain>) -> Self {
        let __this: Chain = Self {
            v: (o.with(|__s| __s.v) + 1_()),
        };
        field!(o, v).write(0_());
        __this
    }
}
impl Clone for Chain {
    fn clone(&self) -> Self {
        let __src: Value<Chain> = Rc::new(RefCell::new(Chain { v: self.v.clone() }));
        Chain::copy_from(__src.as_pointer())
    }
}
pub fn consume_0(mut c: Chain) -> () {
    return c.v;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Chain> = Rc::new(RefCell::new(Chain::new({ 1_() })));
    ({ ChainImpl::add_4(&({ ChainImpl::add_4(&a.as_pointer(), 1_()) }), 1_()) });
    unimplemented!();
    let b0: Value<Chain> = Rc::new(RefCell::new(Chain::new({ 5_() })));
    let mut b: Chain = Chain::move_from({
        ({ ChainImpl::add_5(&({ ChainImpl::add_5(&b0.as_pointer(), 1_()) }), 1_()) })
    });
    unimplemented!();
    let c: Value<Chain> = Rc::new(RefCell::new(
        ({ ChainImpl::take(&Rc::new(RefCell::new(Chain::new({ 10_() }))).as_pointer()) }),
    ));
    unimplemented!();
    let mut d: Chain = ({ ChainImpl::copy(&c.as_pointer()) });
    unimplemented!();
    let g: Value<Chain> = Rc::new(RefCell::new(Chain::new({ 20_() })));
    unimplemented!();
    let e: Value<Chain> = Rc::new(RefCell::new(Chain::new({ 30_() })));
    let mut f: Chain = ({ ChainImpl::take(&e.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub trait ChainImpl {
    fn add_4(&self, n: ()) -> Ptr<Chain>;
    fn add_5(&self, n: ()) -> Ptr<Chain>;
    fn take(&self) -> Chain;
    fn copy(&self) -> Chain;
    fn self_(&self) -> Ptr<Chain>;
}
impl ChainImpl for Ptr<Chain> {
    fn add_4(&self, mut n: ()) -> Ptr<Chain> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).clone();
    }
    fn add_5(&self, mut n: ()) -> Ptr<Chain> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).clone();
    }
    fn take(&self) -> Chain {
        return Chain::move_from({ (*self).clone() });
    }
    fn copy(&self) -> Chain {
        return Chain::copy_from({ (*self).clone() });
    }
    fn self_(&self) -> Ptr<Chain> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
