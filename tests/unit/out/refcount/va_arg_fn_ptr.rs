extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: ()) -> () {
    return (x * x);
}
pub fn negate_1(mut x: ()) -> () {
    return -x;
}
pub fn add_2(mut a: (), mut b: ()) -> () {
    return (a + b);
}
pub fn apply_unary_3(x: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let x: Value<()> = Rc::new(RefCell::new(x));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut fn_: FnPtr<fn(()) -> ()> = (*ap.borrow_mut()).arg::<FnPtr<fn(()) -> ()>>();
    let mut result: () = ({ fn_.call((*x.borrow())) });
    return result;
}
pub fn apply_binary_4(mut a: (), b: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut fn_: FnPtr<fn((), ()) -> ()> = (*ap.borrow_mut()).arg::<FnPtr<fn((), ()) -> ()>>();
    let mut result: () = ({ fn_.call(a, (*b.borrow())) });
    return result;
}
pub fn not_supported_5(mut ctx: (), mut fn_: FnPtr<fn(()) -> ()>, mut extra: ()) -> () {
    &(ctx);
    &(fn_);
    &(extra);
    return -3_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let dummy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut vu: FnPtr<fn((), Box<[VaArg]>) -> ()> =
        FnPtr::<fn((), Box<[VaArg]>) -> ()>::new(apply_unary_3);
    unimplemented!();
    let mut vb: FnPtr<fn((), (), Box<[VaArg]>) -> ()> =
        FnPtr::<fn((), Box<[VaArg]>) -> ()>::new(apply_unary_3)
            .cast::<fn((), (), Box<[VaArg]>) -> ()>();
    vu = vb.cast::<fn((), Box<[VaArg]>) -> ()>();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
