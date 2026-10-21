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
pub struct Val {
    #[offset(0)]
    pub x: (),
}
pub fn sum_0(mut a: Val, mut b: Val) -> () {
    return (a.x + b.x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let total: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let tally: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let total: Ptr<()> = total.as_pointer();
        },
        || {
            total.write({
                (total.read()).wrapping_add(
                    ((::std::mem::size_of::<()>() as ())
                        .wrapping_add((::std::mem::size_of::<()>() as ()))
                        as ()),
                )
            });
        },
        || {
            total.write({
                (total.read()).wrapping_add(
                    ((::std::mem::size_of::<()>() as ())
                        .wrapping_add((::std::mem::size_of::<()>() as ()))
                        as ()),
                )
            });
        }
    )));
    ({ (*tally.borrow()).spec::<fn()>(0).call() });
    ({ (*tally.borrow()).spec::<fn()>(1).call() });
    unimplemented!();
    let v: Value<Val> = Rc::new(RefCell::new(Val { x: 5_() }));
    let acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pick: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Ptr<Val> = v.as_pointer();
            let acc: Ptr<()> = acc.as_pointer();
        },
        || {
            {
                let __rhs = ({
                    let _a: Val = (*v.upgrade().deref()).clone();
                    let _b: Val = (*v.upgrade().deref()).clone();
                    sum_0(_a, _b)
                });
                acc.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        || {
            {
                let __rhs = ({
                    let _a: Val = (*v.upgrade().deref()).clone();
                    let _b: Val = (*v.upgrade().deref()).clone();
                    sum_0(_a, _b)
                });
                acc.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        || {
            {
                let __rhs = ({
                    let _a: Val = (*v.upgrade().deref()).clone();
                    let _b: Val = (*v.upgrade().deref()).clone();
                    sum_0(_a, _b)
                });
                acc.with_mut(|__v| *__v = *__v + __rhs)
            };
        }
    )));
    ({ (*pick.borrow()).spec::<fn()>(0).call() });
    ({ (*pick.borrow()).spec::<fn()>(1).call() });
    ({ (*pick.borrow()).spec::<fn()>(2).call() });
    unimplemented!();
    let cast_to: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: ()| -> () {
            return ((x as ()) / 2_());
        },
        |mut x: ()| -> () {
            return (x / 2_());
        }
    )));
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
