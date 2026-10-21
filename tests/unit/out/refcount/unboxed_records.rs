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
pub struct Inner {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Cookie {
    #[offset(0)]
    pub x: (),
    #[offset(8)]
    #[byte_size(8)]
    pub data: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub in_: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub inner: Inner,
    #[offset(32)]
    #[byte_size(16)]
    pub arr: Box<[()]>,
}
impl Default for Cookie {
    fn default() -> Self {
        {
            Cookie {
                x: unimplemented!(),
                data: Ptr::<()>::null(),
                in_: Ptr::<()>::null(),
                inner: <Inner>::default(),
                arr: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub n: (),
}
impl Default for Counter {
    fn default() -> Self {
        {
            Counter {
                n: unimplemented!(),
            }
        }
    }
}
pub fn set_0(mut p: Ptr<()>, mut v: ()) {
    p.write({ v });
}
pub fn get_1(c: Ptr<Cookie>) -> () {
    return c.with(|__s| __s.x);
}
pub fn first_2(mut p: Ptr<()>) -> () {
    return (elem!(p, unimplemented!()).read());
}
pub fn consume_3(cookie: Ptr<Cookie>) -> () {
    let mut c: Cookie = (*cookie.upgrade().deref()).clone();
    let mut sum: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (i < 4_()) {
        elem!(c.data, i).write({ ({ (elem!(c.in_, i).read()) } * { c.x }) });
        sum += { (elem!(c.data, i).read()) };
        i.prefix_inc();
    }
    c.x = sum;
    c.inner.a += { c.x };
    c.arr[(1_()) as usize] = { c.inner.a };
    let __rhs = ({ c.arr[(1_()) as usize] } + { ({ first_2((c.in_).clone()) }) });
    c.data.write(__rhs);
    ({
        let _p: Ptr<()> = c.data.offset((1_()) as isize);
        let _v: () = ({ first_2((c.data).clone()) });
        set_0(_p, _v)
    });
    return (c.x + c.inner.b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let in_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    let cookie: Value<Cookie> = Rc::new(RefCell::new(<Cookie>::default()));
    (*cookie.borrow_mut()).x = 2_();
    (*cookie.borrow_mut()).data = (data.as_pointer() as Ptr<()>);
    (*cookie.borrow_mut()).in_ = (in_.as_pointer() as Ptr<()>);
    (*cookie.borrow_mut()).inner.a = 1_();
    (*cookie.borrow_mut()).inner.b = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut local: Inner = <Inner>::default();
    local.a = 4_();
    local.b = { (local.a * 2_()) };
    local.a.postfix_inc();
    unimplemented!();
    let field_addr: Value<Inner> = Rc::new(RefCell::new(Inner {
        a: unimplemented!(),
        b: unimplemented!(),
    }));
    ({ set_0((field_ptr!(field_addr.as_pointer(), b)), 7_()) });
    unimplemented!();
    let counter: Value<Counter> = Rc::new(RefCell::new(<Counter>::default()));
    ({ CounterImpl::inc(&counter.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub trait CounterImpl {
    fn inc(&self);
}
impl CounterImpl for Ptr<Counter> {
    fn inc(&self) {
        field!((*self), n).with_mut(|__v| __v.prefix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
