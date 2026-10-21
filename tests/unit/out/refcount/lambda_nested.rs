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
    let x: Value<()> = Rc::new(RefCell::new(10_()));
    let outer: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let x: Ptr<()> = x.as_pointer();
        },
        |y: ()| -> () {
            let y: Value<()> = Rc::new(RefCell::new(y));
            let inner: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
                {
                    let x: Ptr<()> = (x).clone();
                    let y: Value<()> = Rc::new(RefCell::new((*y.borrow())));
                },
                |mut z: ()| -> () {
                    return (((x.read()) + (*y.borrow())) + z);
                }
            )));
            return ({ (*inner.borrow()).call(1_()) });
        }
    )));
    unimplemented!();
    (*x.borrow_mut()) = 100_();
    unimplemented!();
    let s: Value<S> = Rc::new(RefCell::new(S { v: 5_() }));
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn nested_this(&self) -> ();
}
impl SImpl for Ptr<S> {
    fn nested_this(&self) -> () {
        let outer: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            |y: ()| -> () {
                let y: Value<()> = Rc::new(RefCell::new(y));
                let inner: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
                    {
                        let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*this_.borrow()).clone()));
                        let y: Value<()> = Rc::new(RefCell::new((*y.borrow())));
                    },
                    |mut z: ()| -> () {
                        return (((*this_.borrow()).clone().with(|__s| __s.v) + (*y.borrow())) + z);
                    }
                )));
                return ({ (*inner.borrow()).call(1_()) });
            }
        )));
        return ({ (*outer.borrow()).call(20_()) });
    }
}
pub fn __cpp2rust_init_globals() {}
