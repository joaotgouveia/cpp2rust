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
pub struct context {
    #[offset(0)]
    pub verbose: (),
    #[offset(4)]
    pub last_error: (),
}
pub fn set_error_0(mut ctx: Ptr<context>, fmt: Ptr<()>, __args: Box<[VaArg]>) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let fmt: Value<Ptr<()>> = Rc::new(RefCell::new(fmt));
    if (ctx.with(|__s| __s.verbose) != 0) {
        let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
        (*ap.borrow_mut()) = __args.as_pointer();
        let __rhs = (*ap.borrow_mut()).arg::<()>();
        field!(ctx, last_error).write(__rhs);
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct value {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl value {
    pub fn i(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn l(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for value {
    fn default() -> Self {
        value {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
pub fn pick_1(use_long: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let use_long: Value<()> = Rc::new(RefCell::new(use_long));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let v: Value<value> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<value>()));
    if ((*use_long.borrow()) != 0) {
        return (value::l(v.as_pointer()).read());
    }
    return (value::i(v.as_pointer()).read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let ctx: Value<context> = <Value<context>>::default();
    (*ctx.borrow_mut()).verbose = 1_();
    (*ctx.borrow_mut()).last_error = unimplemented!();
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::<()>::from_string_literal(b"error %d"),
            Box::new([(42_()).into()]),
        )
    });
    unimplemented!();
    (*ctx.borrow_mut()).verbose = unimplemented!();
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::<()>::from_string_literal(b"error %d"),
            Box::new([(99_()).into()]),
        )
    });
    unimplemented!();
    let v: Value<value> = <Value<value>>::default();
    value::l(v.as_pointer()).write((1_() << 40_()));
    unimplemented!();
    value::i(v.as_pointer()).write(7_());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
