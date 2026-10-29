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
    pub n: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { n: 1_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn twice(&self) -> ();
    fn modify_copy(&self) -> ();
    fn snapshot(&self) -> ();
    fn mixed(&self, k: ()) -> ();
}
impl SImpl for Ptr<S> {
    fn twice(&self) -> () {
        return ((*self).with(|__s| __s.n) * 2_());
    }
    fn modify_copy(&self) -> () {
        let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
            },
            || -> () {
                {
                    field!(this_.as_pointer(), n).with_mut(|__v| *__v = *__v + 10_())
                };
                return this_.as_pointer().with(|__s| __s.n);
            }
        )));
        let mut r: () = ({ (*f.borrow()).call() }).clone();
        return ((r * 100_()) + (*self).with(|__s| __s.n));
    }
    fn snapshot(&self) -> () {
        let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
            },
            || -> () {
                return ({ SImpl::twice(&this_.as_pointer()) });
            }
        )));
        field!((*self), n).write(99_());
        return ({ (*f.borrow()).call() }).clone();
    }
    fn mixed(&self, k: ()) -> () {
        let k: Value<()> = Rc::new(RefCell::new(k));
        let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
                let k: Value<()> = Rc::new(RefCell::new((*k.borrow())));
            },
            || -> () {
                return (this_.as_pointer().with(|__s| __s.n) + (*k.borrow()));
            }
        )));
        field!((*self), n).write(0_());
        return ({ (*f.borrow()).call() }).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
