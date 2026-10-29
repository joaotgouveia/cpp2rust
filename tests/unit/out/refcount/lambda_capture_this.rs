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
pub struct S {
    #[offset(0)]
    pub n: (),
    #[offset(4)]
    pub step: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S {
        n: 0_(),
        step: 2_(),
    }));
    ({ SImpl::bump(&s.as_pointer(), 3_()) });
    unimplemented!();
    ({ SImpl::bump_via_method(&s.as_pointer(), 4_()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn add(&self, k: ());
    fn scaled(&self) -> ();
    fn bump(&self, by: ());
    fn bump_via_method(&self, by: ());
    fn read_scaled(&self) -> ();
}
impl SImpl for Ptr<S> {
    fn add(&self, mut k: ()) {
        {
            let __rhs = k;
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    fn scaled(&self) -> () {
        return ((*self).with(|__s| __s.n) * (*self).with(|__s| __s.step));
    }
    fn bump(&self, mut by: ()) {
        let inc: Value<FnPtr<fn(())>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            |mut k: ()| {
                {
                    let __rhs = k;
                    field!((*this_.borrow()).clone(), n).with_mut(|__v| *__v = *__v + __rhs)
                };
            }
        )));
        ({ (*inc.borrow()).call(by) });
        ({ (*inc.borrow()).call(by) });
    }
    fn bump_via_method(&self, mut by: ()) {
        let inc: Value<FnPtr<fn(())>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            |mut k: ()| {
                ({ SImpl::add(&(*this_.borrow()).clone(), k) });
            }
        )));
        ({ (*inc.borrow()).call(by) });
    }
    fn read_scaled(&self) -> () {
        let get: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            || -> () {
                return ({ SImpl::scaled(&(*this_.borrow()).clone()) });
            }
        )));
        return ({ (*get.borrow()).call() }).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
