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
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let base: Value<()> = Rc::new(RefCell::new(10_()));
    let add_base: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let base: Ptr<()> = base.as_pointer();
        },
        |mut x: ()| -> () {
            return (x + (base.read()));
        }
    )));
    unimplemented!();
    (*base.borrow_mut()) = 100_();
    unimplemented!();
    let s: Value<S> = Rc::new(RefCell::new(S { x: 1_(), y: 2_() }));
    let sum: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let s: Ptr<S> = s.as_pointer();
        },
        || -> () {
            return (s.with(|__s| __s.x) + s.with(|__s| __s.y));
        }
    )));
    unimplemented!();
    (*s.borrow_mut()).x = 50_();
    unimplemented!();
    let counter: Value<()> = Rc::new(RefCell::new(0_()));
    let bump: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let counter: Ptr<()> = counter.as_pointer();
        },
        || {
            counter.with_mut(|__v| __v.postfix_inc());
        }
    )));
    ({ (*bump.borrow()).call() });
    ({ (*bump.borrow()).call() });
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 1_(), 2_(), 0_()])));
    let swap: Value<FnPtr<fn((), ())>> = Rc::new(RefCell::new(lambda!(
        {
            let arr: Ptr<()> = arr.as_pointer();
        },
        |mut i: (), mut j: ()| {
            let mut t: () = (elem!((arr), j).read());
            elem!((arr), j).write({ (elem!((arr), i).read()) });
            elem!((arr), i).write(t);
        }
    )));
    ({ (*swap.borrow()).call(0_(), 3_()) });
    unimplemented!();
    unimplemented!();
    let total: Value<()> = Rc::new(RefCell::new(0_()));
    let add: Value<FnPtr<fn(())>> = Rc::new(RefCell::new(lambda!(
        {
            let t: Ptr<()> = total.as_pointer();
        },
        |mut x: ()| {
            {
                let __rhs = { x };
                t.with_mut(|__v| *__v = *__v + __rhs)
            };
        }
    )));
    ({ (*add.borrow()).call(2_()) });
    ({ (*add.borrow()).call(3_()) });
    unimplemented!();
    let set_y: Value<FnPtr<fn(())>> = Rc::new(RefCell::new(lambda!(
        {
            let y: Ptr<()> = field_ptr!(s.as_pointer(), y);
        },
        |mut v: ()| {
            y.write({ v });
        }
    )));
    ({ (*set_y.borrow()).call(9_()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
