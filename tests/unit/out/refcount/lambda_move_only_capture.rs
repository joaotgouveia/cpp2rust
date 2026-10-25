extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<()>,
}
impl Owner {
    pub fn new(mut v: ()) -> Self {
        Self { p: Ptr::alloc(v) }
    }
    pub fn move_from(o: Ptr<Owner>) -> Self {
        let __this: Owner = Self {
            p: o.with(|__s| __s.p.clone()),
        };
        field!(o, p).write(Ptr::<()>::null());
        __this
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let o: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 5_() })));
    let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let h: Value<Owner> = Rc::new(RefCell::new(Owner::move_from({ o.as_pointer() })));
        },
        || -> () {
            return ({ (*h.borrow()).p.clone() }.read());
        }
    )));
    let _dtor_f = ScopedDestructor::new(&f, |__p| __p.with(|__f| __f.destroy()));
    unimplemented!();
    unimplemented!();
    let g: Value<()> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    let _dtor_g = ScopedDestructor::new(&g, |__p| __p.with(|__f| __f.destroy()));
    unimplemented!();
    let total: Value<()> = Rc::new(RefCell::new(0_()));
    let consume: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let h: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 7_() })));
            let total: Ptr<()> = total.as_pointer();
        },
        || {
            {
                let __rhs = { ({ (*h.borrow()).p.clone() }.read()) };
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
            { (*h.borrow()).p.clone() }.write(0_());
        }
    )));
    let _dtor_consume = ScopedDestructor::new(&consume, |__p| __p.with(|__f| __f.destroy()));
    ({ (*consume.borrow()).call() });
    ({ (*consume.borrow()).call() });
    unimplemented!();
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 9_() })));
    let _dtor_o2 = ScopedDestructor::new(&o2, |__p| __p.destructor());
    let t: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(({ OwnerImpl::take(&o2.as_pointer()) })));
    let _dtor_t = ScopedDestructor::new(&t, |__p| __p.with(|__f| __f.destroy()));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait OwnerImpl {
    fn destructor(&self);
    fn take(&self) -> FnPtr<fn() -> ()>;
}
impl OwnerImpl for Ptr<Owner> {
    fn destructor(&self) {
        (*self).with(|__s| __s.p.clone()).delete();
    }
    fn take(&self) -> FnPtr<fn() -> ()> {
        return lambda!(
            {
                let self_: Value<Owner> =
                    Rc::new(RefCell::new(Owner::move_from({ (*self).clone() })));
            },
            || -> () {
                return ({ (*self_.borrow()).p.clone() }.read());
            }
        );
    }
}
pub fn __cpp2rust_init_globals() {}
