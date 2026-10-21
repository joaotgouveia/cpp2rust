extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let twice: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: ()| -> () {
            return (x + x);
        },
        |mut x: ()| -> () {
            return (x + x);
        }
    )));
    unimplemented!();
    unimplemented!();
    let base: Value<()> = Rc::new(RefCell::new(10_()));
    let add_base: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let base: Value<()> = Rc::new(RefCell::new((*base.borrow())));
        },
        |mut x: ()| -> () {
            return (x + (*base.borrow()));
        },
        |mut x: ()| -> () {
            return (x + (*base.borrow()));
        }
    )));
    unimplemented!();
    unimplemented!();
    let total: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let accumulate: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let total: Ptr<()> = total.as_pointer();
        },
        |mut x: (), mut y: ()| {
            {
                let __rhs = (x * y);
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        |mut x: (), mut y: ()| {
            total.write({ (total.read()).wrapping_add((x).wrapping_mul(y)) });
        }
    )));
    ({
        (*accumulate.borrow())
            .spec::<fn((), ())>(0)
            .call(2_(), 3_())
    });
    ({
        (*accumulate.borrow())
            .spec::<fn((), ())>(1)
            .call(4_(), 5_())
    });
    unimplemented!();
    let sub: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: (), mut y: ()| -> () {
            return (x - y);
        },
        |mut x: (), mut y: ()| -> () {
            return (x - y);
        }
    )));
    unimplemented!();
    unimplemented!();
    let mixed: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let base: Value<()> = Rc::new(RefCell::new((*base.borrow())));
        },
        |mut x: (), mut y: ()| -> () {
            return ((x * y) + (*base.borrow()));
        },
        |mut x: (), mut y: ()| -> () {
            return ((x * y) + (*base.borrow()));
        }
    )));
    unimplemented!();
    unimplemented!();
    let unused: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
    &(*unused.borrow_mut());
    let outer: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let base: Value<()> = Rc::new(RefCell::new((*base.borrow())));
        },
        |mut y: ()| -> () {
            let inner: Value<FnPtr<Generic>> =
                Rc::new(RefCell::new(lambda!(Generic, {}, |mut z: ()| -> () {
                    return (z + 1_());
                })));
            return (({ (*inner.borrow()).spec::<fn(()) -> ()>(0).call(y) }) + (*base.borrow()));
        }
    )));
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
